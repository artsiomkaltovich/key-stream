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

Criterion cannot express this — `iter_custom` returns exactly one `Duration` per benchmark. So
this becomes a second bench target, `benches/phases.rs` with `harness = false`, running its own
warmup and fixed sample count:

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

### The matrix: 4 modes × 5 distributions × 2 value types = 40 cells, plus calibration

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

**Two value types: `V = u64` and `V = String`.** `u64` alone is not a sufficient axis. It has
`needs_drop() == false`, so the broadcast ring never drops a value on eviction and the whole
destructor path — the one hazard A lives on — is invisible. It also has no allocation cost. A
scratch measurement put a hot single-key send at ~32 ns for `u64` and ~50 ns for `String`, so the
value type moves the number more than most of the matrix does. Doubling the cells is worth it.

**Three `send` implementations in the `send_with_receivers` phase.** This is how the hazard-A fix
gets chosen, and it is the reason that phase exists in this form:

| Variant | What it does | Isolated cost of the handle |
|---|---|---|
| `guard_held` | today's code — the guard spans the broadcast. **Unsafe**, carried only as the baseline | — |
| `clone_sender` | clone `broadcast::Sender` out, release guard, broadcast | 16.4 ns |
| `rc_sender` | map holds `Rc`/`Arc<broadcast::Sender<V>>`; clone that instead | `Rc` 4.1 ns / `Arc` 9.6 ns |

Those are *isolated* latencies from a scratch bench. An in-context run put `clone_sender` at only
~2 ns over `guard_held`, because the clone's atomics overlap the broadcast's own lock traffic.
The whole point of measuring inside the phase harness is to find out which figure is real —
a 2 ns fix is free, a 16 ns fix is ~30% of a hot send and worth the `Rc` wrapper's allocation
per key.

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

The full matrix is 4 modes × 5 distributions × 2 value types = **40 cells**, and it has to run four
times over (two parents, two spikes). The matrix stays; the harness configuration is what gets cut.

**Where the time actually goes.** [key_stream.rs:225-226](benches/key_stream.rs#L225-L226) sets
`sample_size(40)` and `measurement_time(12s)`, and criterion's **default `warm_up_time` is 3 s per
benchmark ID**. That is a 15 s floor per ID before any useful work happens:

| | IDs | Floor per ID | Per branch | × 4 branches |
|---|---|---|---|---|
| Today's config, 3 distributions | 24 | 15 s | 6 min | 24 min |
| Today's config, 5 distributions | 40 | 15 s | 10 min | **40 min** |
| Retuned config, 5 distributions | 40 | 3.5 s | 2.3 min | **9 min** |

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

**4. `SAMPLE_PASSES` 50 → 15 in the phase harness.** It reports min/median/max, and min-of-N
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

Output: per-phase min/median/max as CSV, keeping the four value columns
[parse_criterion.awk](bench-results/20260802-181501/parse_criterion.awk) emits so the existing
comparison tooling works unchanged, with metadata **prefixed** as extra leading columns: branch,
commit, backend, runtime mode, worker count, `W`, `P`. Metadata goes in the run, not a sidecar —
four series across four branches is exactly the setup where one mislabeled CSV costs the whole run.

**Criterion retained for one group:** `full_cycle` — create → send → recv → drop with everything
inside the timed span — run over **the same cells**, using the same
`{mode}/{distribution}/{value}` benchmark IDs (`full_cycle/1x1/1key_10000msg/u64`,
`full_cycle/4x4/10000key_1msg/String`, …). Identical cell names in both outputs means the two CSVs
join on the cell and each phase breakdown sits next to a statistically-solid end-to-end number for
the same workload.

Why keep it alongside the phase harness: criterion's warmup, outlier detection, and confidence
intervals matter when the deltas being chased are 1-10%, and the hand-rolled harness gives a
median, not a confidence interval. The phase harness answers *where* the time goes; criterion
answers *whether the difference is real*. That is the "two runs":
`cargo bench --bench phases` for the breakdown, `cargo bench --bench key_stream` for the gate.
Drop the second if a plain median proves enough.

`full_cycle` replaces `recv_many_keys_many_messages`, which uses `JoinSet`. The `local` side of
both targets must use `futures::future::join_all` instead: `JoinSet::spawn` requires `Send`.
Same constraint as the tests.

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

Per branch: `cargo bench --bench phases` for the breakdown and
`CARGO_TERM_COLOR=never cargo bench --bench key_stream -- --noplot` for `full_cycle`, both landing
CSV in a new `bench-results/<timestamp>/`. Four series: `bench`, `bench-sync-rwlock`,
`spike-sync-drop-rc`, `spike-sync-drop-arc`.

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
4. **Which hazard-A fix?** Compare `guard_held` / `clone_sender` / `rc_sender` within
  `send_with_receivers`, at `V = String` specifically — the `u64` cells cannot answer it, since
  with `needs_drop() == false` the eviction drop never happens and they measure the guard release
  without the thing it protects against. Keep this decision benchmark-driven. `guard_held`
  remains in the matrix as a valid measured option until the numbers decide whether to keep or
  replace it. If it wins and ships, the hazard becomes a documented contract — `V::drop` must not
  re-enter the stream — and [CHANGELOG.md](CHANGELOG.md) gets a known-limitation entry instead of
  a fix entry. That is a defensible choice for `local`, where the failure is a loud panic; weigh it
  harder for `shared`, where it is a silent hang.
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
| [Cargo.toml](Cargo.toml) | Add `futures = { version = "0.3", features = ["alloc"] }` to `[dev-dependencies]` — `join_all` needs it, and the current suite only compiles by inheriting the feature through criterion. Add the `phases` bench target (`harness = false`). |
| `benches/phases.rs` | New in Phase 0. Single-pass phase breakdown over the 4×3×2 cells; parameterized over both backends in Phase 2 so one run covers all of it. |
| [benches/key_stream.rs](benches/key_stream.rs) | Reduced to `full_cycle` over the same cells, parameterized over both backends. `shared` moves to a multi-thread runtime. |
| [BENCH_DIFFS.md](BENCH_DIFFS.md) | Rules 3-5 now scope to `full_cycle`; record that per-phase costs come from the single-pass harness instead. |
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
6. `cargo bench --bench phases` and `cargo bench --bench key_stream` against the Phase 1 table,
   phase by phase. Both backends should land within noise of their spike numbers; a large move
   means the `Backend` abstraction did not compile away. The two-branch checkout-and-diff workflow
   that produced [comparison.csv](bench-results/20260802-181501/comparison.csv) becomes
   unnecessary — after Phase 2 both backends live in one binary and each run compares them
   directly.
