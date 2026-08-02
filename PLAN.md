# Restructure benches, measure removing the cleanup task, then split `KeyStream` into `local` / `shared`

## Context

The crate ships one `KeyStream`. Two branches diverged from `db60f2a` explore two backings for
the key→`broadcast::Sender` map:

- `bench` — `Rc<RefCell<HashMap>>`, cleanup task via `tokio::task::spawn_local` (needs a
  `LocalSet`), bounds `K: Hash+Eq+Clone+'static` / `V: Clone+'static`. No `Send`, so a non-`Send`
  value like `Rc<T>` works.
- `bench-sync-rwlock` (current branch, current [src/lib.rs](src/lib.rs)) — `Arc<std::sync::RwLock<HashMap>>`,
  cleanup via `tokio::spawn`, bounds add `Send + Sync`.

A review suggested the measured gap might be the *async* lock's waker bookkeeping, since no
critical section holds a lock across `.await`. That swap already happened here (`db60f2a` used
`tokio::sync::RwLock`), and
[comparison.csv](bench-results/20260802-181501/comparison.csv) still shows `bench` ahead on
nearly every row — 1-10% typically, 482% on `send_no_receiver`. So the remaining cost is `Arc`
atomics plus real locking, and both variants are worth shipping as `key_stream::local` and
`key_stream::shared`.

But before locking in that split, one thing is unmeasured. Both variants carry a background
cleanup task: `KeyReceiver::drop` sends its key over an unbounded mpsc channel, and a spawned
task removes the key once `receiver_count()` hits zero. That task is *also* the only reason the
`Rc` variant needs a `LocalSet` — and the `LocalSet` requirement is what forces the two
implementations' tests apart. If keys can be reclaimed directly in `Drop`, the task, the channel,
and the `LocalSet` all disappear together, and the split gets substantially smaller.

Phase 1 measures that — but the current benches can't answer it, because drop cost is smeared
across all six of them (each ends with a `yield_now()` inside the timed window to reap the
cleanup task). So Phase 0 first splits the benches by lifecycle phase, giving drop its own
number. Phase 2 then does the split, with Phase 1's answer folded in.

---

## Phase 0 — split the benches by lifecycle phase

Six isolated criterion groups would each rebuild their own setup — the same lifecycle walked six
times to get six numbers. Instead: **one pass, timestamped at each phase boundary.** A single
lifecycle run yields every phase's cost with no repeated work, and because the phases sum to the
whole, the full-cycle number falls out of the same pass for free.

This lives **inside the criterion run**, in one bench target. `iter_custom` returns a single
`Duration`, but criterion only needs to *receive* one number — nothing stops the closure from
timing each phase internally and pushing the breakdown to a side accumulator:

```rust
b.iter_custom(|iters| {
    rt.block_on(async {
        let mut total = Duration::ZERO;
        for _ in 0..iters {
            let (elapsed, phases) = run_pass_instrumented::<V>(mode, dist).await;
            PHASE_SAMPLES.with(|s| s.borrow_mut().push(phases));
            total += elapsed;
        }
        total
    })
});
```

Criterion gets its statistically-sound total; the breakdown comes from **the same passes**. One
run, no repeated work, and `full_cycle` is the sum of the phases by construction rather than by
convention — which doubles as a self-check: if total minus the phase sum exceeds the ~150 ns of
intra-pass `Instant` overhead, something is mismeasured.

A second `harness = false` target was the original design here, on the reasoning that "criterion
cannot express this". That was wrong — it conflated *returning* one number with *observing* one
number — and it cost a duplicated harness, a two-run workflow, and a requirement that both targets
keep byte-identical cell names so their CSVs join. Merging removes all three.

Three consequences to handle:

- **Warmup iterations land in the accumulator**; nothing distinguishes them from inside the
  closure. Acceptable: measurement runs ~6× more iterations than warmup at 3 s vs 500 ms, so they
  are a slow tail that min ignores and that barely moves the median. Report N per cell.
- **Discarded passes** (cleanup timeout) must still be counted in criterion's returned total, but
  excluded from the phase statistics. Document that asymmetry and print the discard count.
- **The three `send` variants run inside one pass**, back to back on the same stream with capacity
  `3 × messages_per_key + 1`, drained once at the end. One benchmark ID per cell rather than
  three, and the variants are compared under identical map and cache state — which is exactly the
  condition the scratch numbers showed the answer depends on.

One pass timing six phases:

| # | Phase | Normalized by |
|---|---|---|
| 1 | `create` | 1 |
| 2 | `subscribe` | receivers created |
| 3 | `send_with_receivers` | sends |
| 4 | `send_no_receivers` | sends (key absent, map at full size) |
| 5 | `recv` | messages delivered |
| 6 | `drop` | receivers dropped — **includes reclamation** |

`send_no_receivers` stays its own phase rather than a zero-fanout parameter: it misses the map
and returns 0 without touching a broadcast channel — a different path, not a smaller one. The old
482% gap there was measured against an *empty* map, making it near-pure lock overhead; measuring
it with the map at full size is what shows whether that survives realistic conditions.

Each phase is a **loop of N operations** reported as `phase / N`. `Instant::now()` costs ~20-25ns
on macOS and would swamp a 2.4ns send if timestamped per operation; amortized over the loop it is
noise. That is why phases are timed rather than operations.

### The matrix: 4 modes × 5 distributions × 3 value types = 60 cells, plus calibration

Run it in full — every mode, every distribution, both value types, every phase. Nothing is
sampled or dropped to save time; the run-time budget below is met by retuning the harness, not by
thinning the matrix.

Four concurrency modes:

| Mode | Sender tasks | Receivers per key | Isolates |
|---|---|---|---|
| `1x1` | 1 | 1 | Baseline: no contention, no fanout |
| `16x1` | 16 | 1 | Map-lock contention from concurrent writers |
| `1x16` | 1 | 16 | Broadcast delivery cost, uncontended map |
| `4x4` | 4 | 4 | Both at once — the only mode where the map lock and the broadcast channel's internal lock contend simultaneously, and the realistic middle |

Three key distributions, **all 10 000 total sends**, so the cells differ only in how those sends
spread across the map:

| Cell | Keys × messages | What it isolates |
|---|---|---|
| `1key_10000msg` | 1 × 10 000 | Map stays at one entry, permanently cache-hot. Pure broadcast path. |
| `10key_1000msg` | 10 × 1 000 | Still trivially cache-resident, but the lookup is no longer degenerate. |
| `100key_100msg` | 100 × 100 | Map fits in cache but every send is a distinct lookup. The realistic middle: many moderately-active channels. |
| `1000key_10msg` | 1 000 × 10 | Where the map starts outgrowing L1 and reclamation becomes non-trivial. |
| `10000key_1msg` | 10 000 × 1 | Map far past cache, every send a cold lookup. The heaviest reclamation on drop, and the only cell reaching `optimize_dict_mem`'s `shrink_to` (needs capacity > 64). |

A decade sweep rather than three points: the interesting output is *where the curve bends*, and
three samples cannot show a bend. Scratch measurement says `HashMap::get` itself is flat at ~7.5 ns
across this whole range, so any bend that appears is reclamation, allocation, or ring-buffer
pressure — not lookup.

Holding the product constant is what makes this readable: the three cells do identical total work,
so any spread between them is attributable to key distribution alone rather than to message
volume. If the curve turns out nonlinear, re-run at 1 000 and 100 000 total to check scaling.

**Three value types: `V = u64`, `V = String`, and `V = DropValue`.** `u64` alone is not a
sufficient axis. It has `needs_drop() == false`, so the broadcast ring never drops a value on
eviction and the whole destructor path — the one hazard A lives on — is invisible. `String` adds
allocator pressure. `DropValue` adds a lightweight real destructor with an observable side effect
so eviction-drop behavior is measured without allocator noise.

**Three `send` implementations in the `send_with_receivers` phase.** This is how the hazard-A fix
gets chosen, and it is the reason that phase exists in this form:

| Variant | What it does | Atomic RMWs per send | Isolated | **Measured, all 60 cells (mean Δ)** |
|---|---|---|---|---|
| `guard_held` | historical baseline — the guard spans the broadcast. **Unsafe**, kept only via bench helper methods | 0 | — | baseline |
| `clone_sender` | clone `broadcast::Sender` out, release guard, broadcast | 4 | 16.4 ns | **+5.77 ns** |
| `rc_sender` | map entry holds `Rc<broadcast::Sender<V>>`; clone that | 0 (non-atomic) | 4.1 ns | **+1.96 ns** |
| `arc_sender` | map entry holds `Arc<broadcast::Sender<V>>`; clone that | 2 | 9.6 ns | **+2.08 ns** |

Full matrix, `min` per cell, arm64, zero discards —
[bench-results/local-full-variants/](bench-results/local-full-variants/). Medians track the means
to within 0.1 ns; `clone_sender` never fell below +4.16 in any cell.

These numbers are correct for arm64, but not universal. The marginal cost of the first atomic pair
was ~0.1 ns on Apple Silicon, while CI x86 runs showed ~2.6-10.5 ns depending on CPU model. So Q4b
(`arc_sender` vs `clone_sender` for the shared backend) is architecture- **and** CPU-model
dependent and cannot be closed until measurements are pinned to a known CPU.

**`Rc` vs `Arc` is a coin flip.** Head-to-head over 60 cells: mean +0.12 ns, median +0.10,
range −0.91 … +0.80, `arc` ahead in 18 of 60. The atomic refcount costs nothing measurable at this
scale — so the handle choice does **not** argue for or against the local/shared split.

**The `num_tx` increment is the entire cost.** `Sender::clone` is `Arc::clone` plus a `num_tx`
bump. Every variant that skips that bump lands at ~2 ns; the one that pays it costs ~5.8 ns. Atomic
count does not order the results — what separates cleanly is whether tokio's `Sender::clone` is
called at all.

> **Correction.** An earlier single-cell reading (`1x1/1key_10000msg/u64`) put `rc_sender` at
> +0.03 ns and `arc_sender` at −0.23 ns, and was recorded here as "the handle variants are free".
> That was an artifact of the hottest possible map — one entry, permanently cache-resident. Across
> the sweep both cost ~2 ns. The direction held; the magnitude did not. Single-cell results have now
> been wrong twice (see also the fixed-variant-order bias); read the matrix, not a cell.

**Consequence:** the safe options cost ~2 ns on a 45-100 ns send — cheap, not free. `clone_sender`,
which currently ships, is ~3× that. Contention remains unmeasured: every number above is
single-threaded, so `16x1` and `4x4` show scheduling interleave rather than lock contention, and
the case where `guard_held` blocks writers cannot appear until a real shared backend runs on a
multi-thread runtime.

Current branch status: shipping `KeySender::send` uses `clone_sender` — the most expensive of the
correct options — and `guard_held` survives only as a bench helper.

### Why `arc_sender` must be measured, not inferred

`rc_sender`'s +0.3 ns says nothing about the `shared` backend. `Rc::clone` is a non-atomic
increment; `Arc::clone` is two atomic RMWs. That is a category change, not a scaling factor, and it
is the only thing separating `arc_sender` from `clone_sender`.

`arc_sender` is measurable **on the local backend right now**, because the handle type stored in the
map is orthogonal to the cell type guarding it — an `Arc<Sender<V>>` sits inside an
`Rc<RefCell<HashMap<…>>>` perfectly well. Running all four variants in one pass, under one lookup
and one rotation schedule, isolates handle-clone cost exactly. What it cannot show is contention;
that needs the real `shared` backend on a multi-thread runtime.

### Q4 is two questions, not one

The two backends can legitimately pick different winners, and one option is not even available on
both:

- **`local`** — all four variants are candidates. Hazard A fails loudly here (`RefCell` panic), so
  `guard_held` is at least arguable if the fix proves expensive.
- **`shared`** — `guard_held` is **not shippable at any speed**: hazard A there is a silent
  deadlock, not a panic. And holding a read guard across the broadcast blocks every writer, so
  `subscribe` and reclamation stall behind it; the `16x1` and `4x4` modes exist to expose that and
  have never been run against a shared backend. The real contest is `arc_sender` vs `clone_sender`.

**Consequence for Phase 2.** If the winners differ, the map's value type differs per backend —
`HashMap<K, Rc<Sender<V>>>` versus `HashMap<K, Sender<V>>` — so `Backend` must abstract over the
*stored handle type* as well as the cell. The `core.rs` sketch below still hardcodes
`Map<K, V> = HashMap<K, broadcast::Sender<V>>`; that has to become an associated type
(`type Handle: Clone + Deref<Target = broadcast::Sender<V>>`) unless both backends converge.

Two scratch results worth recording so they are not re-derived: `HashMap<i32, _>::get` is **flat
at ~7.5 ns from 1 to 10 000 entries** (SipHash, not cache misses — the map stays resident), and
`Sender::clone` costs ~2× `Arc::clone` purely because tokio's `Clone` bumps a second `num_tx`
counter alongside the `Arc`.

Cell name is `{mode}/{distribution}/{value}` — `1x1/1key_10000msg/u64`,
`4x4/10000key_1msg/String`, and so on. The `send_with_receivers` phase appends the send variant
(`.../clone_sender`); no other phase varies by it.

**Runtime, and why the existing numbers are incomplete.** [benches/key_stream.rs:8](benches/key_stream.rs#L8)
builds `new_current_thread`, so every number in
[comparison.csv](bench-results/20260802-181501/comparison.csv) measured `Arc<RwLock>` with zero
contention — all of its cost, none of its benefit. The multi-sender modes only mean anything on a
multi-thread runtime. So: `shared` runs on `new_multi_thread` in every mode (its deployment
reality; `1x1` there still pays atomics but contends with nothing), and `local` runs on
`current_thread` always, spawning concurrent senders via `join_all` rather than `tokio::spawn`
since it has no other option. [BENCH_DIFFS.md](BENCH_DIFFS.md) rule 6 already sanctions this
asymmetry, and it is the point: `local`'s ceiling in `16x1` is one core, and the matrix should
show where that stops being a good trade.

**These are deployment-mode numbers, not data-structure-overhead numbers, and every table and CSV
column must say so.** `local` on current-thread versus `shared` on multi-thread compares two ways
of shipping the crate, which is the right question — but it is trivially misread later as
"`Rc<RefCell>` is N% faster than `Arc<RwLock>`", which it does not show.

**Plus one calibration cell: `shared` on `current_thread`, `1x1` only.** With the runtime held
equal to `local`'s, the remaining difference *is* pure backend overhead — the atomics and the lock,
with contention removed. It is also the only cell comparable to
[comparison.csv](bench-results/20260802-181501/comparison.csv), which was entirely current-thread;
without it the new run has no continuity with the old and every prior number becomes unusable.
One extra cell, labelled `1x1-calibration/*`.

The `drop` phase is the one Phase 1 turns on, and its span must cover reclamation, not just the
`drop` call: under the task-based design that means waiting until `n_keys()` reaches zero; under
the sync-drop spike there is nothing to wait for. That asymmetry is the measurement — same work,
relocated.

**The reclamation wait must be bounded, and the bound must be a hard failure — not a truncation.**
An unbounded yield loop lets a rare scheduler stall skew the median or hang a sample. But
truncating a slow wait is worse than hanging: it would record a *partial* drop cost, systematically
under-reporting the task-based variant and biasing the whole Phase 1 comparison toward keeping the
task. So: cap on **both** yield count and wall time, whichever trips first, and on trip discard the
sample and emit a visible `FAILED` row rather than a number. [src/lib.rs:593](src/lib.rs#L593) sets
the precedent — the existing `test_shrink_dict` already caps at 10 yields.

### Run-time budget — it is the criterion config, not the send volume

The full matrix is 4 modes × 5 distributions × 3 value types = **60 cells**, and it has to run four
times over (two parents, two spikes). The matrix stays; the harness configuration is what gets cut.

**Where the time actually goes.** [key_stream.rs:225-226](benches/key_stream.rs#L225-L226) sets
`sample_size(40)` and `measurement_time(12s)`, and criterion's **default `warm_up_time` is 3 s per
benchmark ID**. That is a 15 s floor per ID before any useful work happens:

| | IDs | Floor per ID | Per branch | × 4 branches |
|---|---|---|---|---|
| Today's config, 3 distributions | 36 | 15 s | 9 min | 36 min |
| Today's config, 5 distributions | 60 | 15 s | 15 min | **60 min** |
| Retuned config, 5 distributions | 60 | 3.5 s | 3.5 min | **14 min** |

The 10 000-send workload is not the problem. At a few milliseconds per cycle it is a rounding error
next to 3 s of warmup repeated 40 times. Three configuration changes recover the whole difference
without touching a single cell:

**1. `warm_up_time` 3 s (default) → 500 ms.** Criterion's default is tuned for nanosecond-scale
benchmarks that need thousands of iterations to settle. Here one `iter_custom` iteration is already
milliseconds, so a few iterations warm the caches and allocator completely. Across 40 IDs this
alone removes ~100 s per branch of pure waiting.

**2. `measurement_time` 12 s → 3 s, `sample_size` 40 → 20.** With multi-millisecond iterations, 12 s
buys precision far beyond what a 1-10% comparison needs, and 20 samples still supports a confidence
interval. (Criterion's floor is 10 if this needs to go further.)

**3. Set `SamplingMode::Flat` explicitly.** Under the default `Auto`, criterion may use *Linear*
sampling, where sample *i* runs *i* iterations — 20 samples then costs 1+2+…+20 = 210 iterations
instead of 20. `Auto` is supposed to switch to Flat for slow benchmarks, but it decides from
estimates; setting it explicitly removes the guesswork and is correct for `iter_custom` with
millisecond iterations. Potentially a further ~10× on the actual work, on top of the floor above.

**4. `SAMPLE_PASSES` 50 → 15 in the phase harness.** It reports min/median/p95, and min-of-N
converges quickly; 50 buys little over 15. This target is not the bottleneck — apply it last.

Apply 1-3 before anything else: they are three lines of configuration and recover ~75% of the run.

### Harness parameters

Fixed, recorded in the output, not left to the machine:

| Parameter | Value | Why |
|---|---|---|
| Warmup passes `W` | 3 | Each pass is milliseconds; three is enough to settle the allocator and caches |
| Sample passes `P` | 50 | Raise for `1key_10000msg` if the spread is wide |
| Tokio worker threads | **fixed**, not `available_parallelism()` | Comparing four branches — machine realism is worthless if the worker count drifts between series. Must be ≥ 16 or the `16x1` mode oversubscribes and measures the scheduler instead of the lock |
| Reclamation bound | 10 yields **or** 50 ms | Per above |

Record the core count too. `16x1` on a machine with fewer than 16 cores is a different experiment.

Output: per-phase min/median/p95 as CSV with an explicit row kind column (`phase` / `failed`),
plus metadata **prefixed** as leading columns: branch, commit, backend, runtime mode, worker
count, `W`, `P`. This intentionally supersedes the old `parse_criterion.awk` schema rather than
pretending backward compatibility.

**One group, `full_cycle`**, over benchmark IDs `{mode}/{distribution}/{value}`
(`full_cycle/1x1/1key_10000msg/u64`, `full_cycle/4x4/10000key_1msg/String`, …). Criterion reports
the end-to-end number for each ID; the side accumulator reports the phase breakdown under the same
cell name. They cannot disagree about which workload they describe, because they are the same
passes — no cross-file join to keep in sync.

Criterion earns its place here: its warmup, outlier detection, and confidence intervals matter when
the deltas being chased are 1-10%, and min/median alone is not a confidence interval. Criterion
answers *whether a difference is real*; the accumulator answers *where the time went*. One run,
`cargo bench --bench key_stream -- --noplot`, produces both.

`full_cycle` replaces `recv_many_keys_many_messages`, which uses `JoinSet`. The `local` side must
use `futures::future::join_all` instead: `JoinSet::spawn` requires `Send`. Same constraint as the
tests.

Applied to **both** `bench` and `bench-sync-rwlock` before any spike branches, so all four Phase 1
series share a harness. Numbers will not be comparable to the existing
[comparison.csv](bench-results/20260802-181501/comparison.csv); that run gets superseded.

---

## Phase 1 — spike: synchronous cleanup in `KeyReceiver::drop`

Two scratch branches, `spike-sync-drop-arc` and `spike-sync-drop-rc`, branched off the Phase 0
work on `bench-sync-rwlock` and `bench` respectively. Same change on each; benches untouched from
Phase 0 so each spike is comparable to its parent.

The change, in [src/lib.rs](src/lib.rs):

- `KeyReceiver` holds the backend handle (`Streams<K, V>`) instead of `UnboundedSender<K>`, and
  its receiver field becomes `Option<broadcast::Receiver<V>>`.
- `Drop for KeyReceiver`: `self.receiver.take()` and drop it **first**, then take the write
  guard, then `if sender.receiver_count() == 0 { remove; optimize_dict_mem }`.
  The `take()` is load-bearing. Fields are not yet dropped when `Drop::drop` runs, so
  `receiver_count()` still counts this receiver; checking `== 1` instead races — two concurrent
  drops can each observe 2, neither removes, and the key leaks.
- Delete `cleanup_keys`, the `unbounded_channel`, `drop_keys_task`, and `Drop for KeyStream`.
  `KeyStream::new` stops spawning, so it no longer panics outside a runtime and the `Rc` variant
  no longer needs a `LocalSet`.
- Tests: drop the `yield_now()` calls; `test_send_after_drop_before_cleanup_runs` and
  `test_resubscribe_before_cleanup_runs` now assert immediate removal rather than deferred.
  On `spike-sync-drop-rc`, delete the `run_on_localset` wrapper entirely — proving the `LocalSet`
  requirement is gone is half the point of the spike.

### Two re-entrancy hazards — both verified by experiment, both fixed here

User code must never run while the map guard is held. Two places violate that. Both were
reproduced against real code in `key-stream-bench`; neither is speculative.

**Hazard A — `send` drops the evicted ring value under the read guard. Pre-existing, not caused
by Phase 1.** `broadcast::Sender::send` does `slot.val = Some(value)`, and that assignment drops
whatever the slot held. If `V::drop` re-enters the stream, `RefCell` panics
(`test_send_reentrant_drop_panics_refcell` reproduces it) and `RwLock` **deadlocks silently** —
strictly worse. Preconditions: the ring wraps, *and* a receiver was lagging on the evicted slot
(a fully-consumed slot is already `None`, because `RecvGuard::drop` clears it), *and* `V::drop`
touches the stream. Narrow, but the shared backend's failure mode is a hung task with no output.

Fix: release the guard before broadcasting, which requires an owned handle to the channel. Two
ways to get one, and **Phase 0 measures both** (see below) rather than guessing — scratch numbers
put the cost anywhere from 2 to 16 ns depending on whether surrounding work hides the atomics.
`send_no_receivers` is untouched either way: a miss has nothing to clone.

A third variant, gating on `needs_drop::<V>()`, is **rejected outright**: it skips the clone only
for value types that cannot run destructors, which is precisely when the hazard cannot occur. It
buys speed exactly where no protection was needed, in exchange for an invisible branch.

**Hazard B — dropping the `broadcast::Receiver` while holding the write guard. Introduced by
Phase 1, and free to fix.** Take-first ordering — `self.receiver.take()` and drop it *before*
acquiring the guard — closes it completely. Verified both ways: the naive ordering panics
(`test_sync_drop_drop_receiver_under_borrow_panics_refcell`), take-first does not.

Take-first is also *sufficient*, which is not obvious. `streams.remove(&key)` drops the last
`broadcast::Sender`, and it would seem that could drop buffered values under the guard — but
`Receiver::drop` drains its unread slots in a loop before returning, so the ring is already empty
by the time the guard is taken. Confirmed with a tracer value: an unread buffered `V` drops during
`Receiver::drop`, never at `Sender::drop`.

**Note for Phase 1's framing:** today's cleanup task is load-bearing for more than scheduling.
Deferring removal through a channel is exactly what makes the removal path re-entrancy-safe.
Removing the task trades that property for the take-first rule — a fine trade, but an explicit one.

**Test hygiene (Phase 0-1 scope):** the hazard tests in `key-stream-bench` should use
`#[should_panic(expected = "already borrowed")]` and stay panic-based. Shared deadlock repro
tests are intentionally out of scope for Phase 0-1. This phase is about measuring cost and
choosing implementation direction, not building a deadlock harness.

### Measurement

One command per branch: `CARGO_TERM_COLOR=never cargo bench --bench key_stream -- --noplot`,
which emits both the `full_cycle` totals and the phase breakdown into a new
`bench-results/<timestamp>/`. Four series: `bench`, `bench-sync-rwlock`, `spike-sync-drop-rc`,
`spike-sync-drop-arc`.

### Decision gate

Five questions, answered from the same table:

1. **Does removing the task pay?** Compare each spike against its parent. The `drop` phase is the
   direct read; `create`, `recv`, and both send phases should be roughly flat, and a move there
   means something other than reclamation changed.
2. **Does it cost anything elsewhere?** `10000key_1msg` is where batching loss would show: the
   task coalesces 10k notifications into one pass, sync drop takes 10k write locks.
   `1key_10000msg` should be untouched — if it moves, the change leaked into the hot path. Watch
   `16x1` and `4x4` specifically: sync drop takes the write lock from N threads instead of
   funneling through one task, so contention could appear there and nowhere else.
3. **Does it shrink the local↔shared gap?** Compare `spike-sync-drop-rc` vs `spike-sync-drop-arc`
   against `bench` vs `bench-sync-rwlock`. If the gap largely closes, the case for shipping two
   backends weakens and Phase 2 may collapse to one. Read `16x1` and `4x4` separately from `1x1`
   — those are the modes where `shared` can actually win, and no prior run has measured them.
4. **Which hazard-A fix — asked separately per backend.** Compare all four variants within
  `send_with_receivers`, at `V = String` and `V = DropValue` specifically. The `u64` cells cannot
  answer it: with `needs_drop() == false` the eviction drop never happens, so they measure the
  guard release without the thing it protects against.

   **4a. `local`.** All four are candidates. `guard_held` stays in the matrix as a measured option
   until the numbers decide. If it wins and ships, the hazard becomes a documented contract —
   `V::drop` must not re-enter the stream — and [CHANGELOG.md](CHANGELOG.md) gets a
   known-limitation entry instead of a fix entry. Defensible here, where the failure is a loud
   `RefCell` panic.

   **4b. `shared`.** `guard_held` is disqualified before any measurement: hazard A there is a
   silent deadlock, and holding a read guard across the broadcast blocks every writer, stalling
   `subscribe` and reclamation behind it. Read it as a reference number only. The decision is
   `arc_sender` vs `clone_sender`, and it must be read from the contended modes (`16x1`, `4x4`) —
   `1x1` cannot show the cost that disqualifies `guard_held`, and no prior run has measured any of
   this on a shared backend.

   Do not let 4a's answer stand in for 4b's. `rc_sender`'s near-zero cost comes from `Rc::clone`
   being non-atomic; the shared equivalent is two atomic RMWs. If the winners differ, see the
   `Backend` associated-type consequence noted in Phase 0.
5. **How much of the gap is the backend, and how much is the runtime?** Read the
   `1x1-calibration` cell — `shared` on current-thread, contention removed. The spread between it
   and `local/1x1` is pure `Arc`-plus-lock overhead; the spread between it and `shared/1x1` on
   multi-thread is what the runtime costs. Every other cell conflates the two.

Report the table before starting Phase 2 rather than proceeding straight through.

### Open Follow-ups From Phase 0 Issues Review (No Decision Yet)

These are tracked as pending choices; they are intentionally not resolved in this document yet.

1. Variant implementation location for hazard-A measurement:
- Option A: add temporary bench-only variants in library code.
- Option B: keep variant logic in shared benchmark module only.

2. `create` phase amortization factor:
- Requirement: amortize clock overhead by looping `KeyStream::new` and dividing by loop count.
- Exact loop count is open and should be selected empirically.

3. Receiver drain ordering in `recv` phase:
- Current receiver-by-receiver order is acceptable for phase totals.
- If per-receiver metrics are added later, reassess with round-robin draining.

---

## Phase 2 — the split

Shape below assumes the task is removed. If Phase 1 says keep it, `Backend` regains a
`spawn_cleanup(rx, backend) -> JoinHandle<()>` method (the one place backend-specific control
flow lives), `KeyReceiverCore` drops its `C` parameter, and the test macro's `local` arm regains
its `LocalSet::new().run_until(…)` wrapper — still in exactly one place.

### Decisions already made

- **No crate-root re-export.** `lib.rs` exposes only `pub mod local` / `pub mod shared`. Breaking
  for 0.10 users (one import line), but neither variant becomes a silent default and both are
  equally visible in rustdoc.
- **`send` / `subscribe` / `n_keys` / `key_capacity` become plain `fn`.** Neither backend awaits
  in them. `recv` stays `async`. Also strips ~80 `.await`s from the test bodies.
- **GATs for the guard types**, so core call sites keep the shape they have in lib.rs today and
  there is no closure indirection relying on inlining.
- **Both re-entrancy fixes carry forward into core**, written once for both backends: `send`
  releases the guard before broadcasting (via whichever handle variant Phase 0 selects), and
  `KeyReceiverCore::drop` takes the receiver before touching the map. Neither is backend-specific.
  If Phase 0 picks `rc_sender`, `Backend`'s `Map<K, V>` alias becomes
  `HashMap<K, Rc<broadcast::Sender<V>>>` on `local` and the `Arc` equivalent on `shared`, which
  means the map's value type joins the trait rather than being fixed in core.

### `src/core.rs` (private `mod core`) — all real logic, exactly once

No mention of `Rc`/`Arc`/`RefCell`/`RwLock`, and no `Send`/`Sync` bound anywhere.

```rust
type Map<K, V> = HashMap<K, broadcast::Sender<V>>;

pub(crate) trait Backend<K, V>: Clone {
    type Read<'a>: Deref<Target = Map<K, V>> where Self: 'a;
    type Write<'a>: DerefMut<Target = Map<K, V>> where Self: 'a;

    fn new_map(map: Map<K, V>) -> Self;
    fn read(&self) -> Self::Read<'_>;
    fn write(&self) -> Self::Write<'_>;
}
```

Contents, ported from the current bodies with `read_streams(&self.streams)` → `self.streams.read()`:

- `KeyStreamCore<C: Backend<K,V>, K, V>` — `new`, `sender`, `n_keys`, `key_capacity`. The bound
  sits on the struct definition so `V` counts as used; no `PhantomData`.
- `KeySenderCore<C: Backend<K,V>, K, V>` — `send`, `subscribe`, `n_keys`, `key_capacity`, `Clone`.
- `KeyReceiverCore<C: Backend<K,V>, K, V>` — `recv`, `try_recv`, `blocking_recv`, `into_stream`,
  and the reclaiming `Drop` from Phase 1.
- `optimize_dict_mem` — unchanged.

`Send`-ness falls out structurally: each `impl Backend for …` carries its own where-clause, so
nothing in core needs a manual `Send` bound. Poison handling stays per-impl — the shared impl
keeps `unwrap_or_else(PoisonError::into_inner)` inside `read`/`write`; `RefCell` has no poisoning
and just borrows. Core never sees it.

### `src/local.rs` and `src/shared.rs` — thin

Each: one `impl Backend<K, V> for <backend type>`, the marker traits, three type aliases, and a
`//!` doc example.

```rust
// shared.rs
pub trait Key: Hash + Eq + Clone + Send + Sync + 'static {}
pub trait Value: Clone + Send + 'static {}   // NOT Sync — matches today's bound exactly
pub type KeyStream<K, V> = KeyStreamCore<Arc<RwLock<Map<K, V>>>, K, V>;
pub type KeySender<K, V> = KeySenderCore<Arc<RwLock<Map<K, V>>>, K, V>;
pub type KeyReceiver<K, V> = KeyReceiverCore<Arc<RwLock<Map<K, V>>>, K, V>;
```

`local.rs` mirrors it with `Rc<RefCell<Map<K,V>>>` and bounds without `Send`/`Sync`. Inherent
methods on the core structs apply through the aliases, so there is no delegation layer. Keeping
per-module `Key`/`Value` preserves the currently-public `key_stream::Key` / `key_stream::Value`
names rather than dropping them.

### Test de-duplication

Bodies live once in `core.rs` as generic async fns over `C: Backend<K, V>`; a macro emits both
wrappers, so any per-backend difference sits in exactly one place.

```rust
macro_rules! dual_backend_test {
    ($name:ident, $body:path, $k:ty, $v:ty) => {
        mod $name {
            use super::*;
            #[tokio::test]
            async fn shared() { $body::<SharedC<$k, $v>>().await; }
            #[tokio::test(flavor = "current_thread")]
            async fn local() { $body::<LocalC<$k, $v>>().await; }
        }
    };
}

async fn test_recv_body<C: Backend<String, String>>() {
    let key_stream = KeyStreamCore::<C, _, _>::new(10);
    let sender = key_stream.sender();
    let mut receiver = sender.subscribe("1".to_string());
    assert_eq!(key_stream.n_keys(), 1);
    sender.send(&"1".to_string(), "value".to_string());
    assert_eq!(receiver.recv().await.unwrap(), "value".to_string());
}
dual_backend_test!(test_recv, test_recv_body, String, String);
```

Explicit `$k`/`$v` rather than `_`, so the generic call has something to anchor inference to.

`test_shrink_dict` is the only body with concurrency. It uses `futures::future::join_all` over
async blocks, **not** `JoinSet::spawn` — `JoinSet::spawn` requires `Send` and will not compile
under the local backend. The `bench` branch already made this switch for that reason. No
per-backend spawn-dispatch helper: the test asserts only that 500 subscriptions exist, most are
dropped, and cleanup catches up, none of which depends on separately-scheduled tasks.

Four tests stay hand-written outside the macro, because each asserts something backend-specific
rather than incidental duplication:

- `test_recv_rc_struct` (local, `V = Rc<MyStruct>`) and `test_recv_arc_struct` (shared,
  `V = Arc<MyStruct>`) — a real capability difference, since only `local` accepts a non-`Send` `V`.
- The two hazard regression tests for `local` with
  `#[should_panic(expected = "already borrowed")]`. Shared deadlock-focused tests are deferred and
  not required for this Phase 0-1 plan.

Net: 19 generic bodies + 4 specials (with local hazard tests), replacing 19 near-identical bodies
per backend.

### Files

| File | Change |
|---|---|
| [src/core.rs](src/core.rs) | New. `Backend`, the three `*Core` types, `optimize_dict_mem`, dual-backend test module. |
| [src/local.rs](src/local.rs) | New. `Backend` impl for `Rc<RefCell<…>>`, `Key`/`Value`, aliases, `//!` docs. |
| [src/shared.rs](src/shared.rs) | New. `Backend` impl for `Arc<RwLock<…>>`, `Key`/`Value`, aliases, `//!` docs. |
| [src/lib.rs](src/lib.rs) | Reduced to crate docs + `mod core; pub mod local; pub mod shared;`, plus a backend-selection guide. |
| [Cargo.toml](Cargo.toml) | Add `futures = { version = "0.3", features = ["alloc"] }` to `[dev-dependencies]` — `join_all` needs it, and the current suite only compiles by inheriting the feature through criterion. |
| [benches/common/mod.rs](benches/common/mod.rs) | Shared workload: modes, distributions, value types, send variants, metadata. |
| [benches/key_stream.rs](benches/key_stream.rs) | The only bench target. `full_cycle` plus the instrumented phase breakdown from the same passes; parameterized over both backends in Phase 2. `shared` moves to a multi-thread runtime. |
| `benches/phases.rs` | **Deleted** — merged into the criterion target. |
| [BENCH_DIFFS.md](BENCH_DIFFS.md) | Rules 3-5 now scope to `full_cycle`; record that per-phase costs come from the same instrumented passes. |
| [README.md](README.md) | Examples → `shared::KeyStream`; add a backend-selection section. |
| [CHANGELOG.md](CHANGELOG.md) | Under `[Unreleased] / Fixed`: `send` no longer runs `V::drop` while holding the map lock (panic on `local`, deadlock on `shared`). Under `Changed`: module split, removed root exports, `async fn` → `fn`, and (if Phase 1 lands) immediate rather than deferred key cleanup. |

## Verification

1. `cargo test` — 19 macro-generated tests run twice, plus the 4 specials. The suite must be
   **green**: the hazard tests assert their failure mode rather than exhibiting it, which is not
   true of the current `key-stream-bench` versions.
2. `cargo test --doc`.
3. `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check`.
4. Negative compile check, run by hand: `shared::KeyStream::<String, Rc<u8>>::new(10)` must be
   rejected for want of `Send`. Confirms the backends differ in capability, not just in name.
5. Hazard-A regression for `local`: a `V` whose `Drop` calls `subscribe` on a new key, driven
  until the ring wraps with a lagging receiver. Use panic-based assertions for this phase.
6. `cargo bench --bench key_stream` against the Phase 1 table, phase by phase. Also check that
   each cell's `full_cycle` total minus its phase sum stays within the ~150 ns of intra-pass
   `Instant` overhead — a larger gap means a phase span is mismeasured. Both backends should land
   within noise of their spike numbers; a large move
   means the `Backend` abstraction did not compile away. The two-branch checkout-and-diff workflow
   that produced [comparison.csv](bench-results/20260802-181501/comparison.csv) becomes
   unnecessary — after Phase 2 both backends live in one binary and each run compares them
   directly.
