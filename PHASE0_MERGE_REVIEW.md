# Phase 0 Merge Review

Review of the merged single-target bench after `benches/phases.rs` was folded into
[benches/key_stream.rs](benches/key_stream.rs). Scope: [benches/common/mod.rs](benches/common/mod.rs),
[benches/key_stream.rs](benches/key_stream.rs), and the `__bench_*` additions in
[src/lib.rs](src/lib.rs).

## What landed well

The merge itself is right. One `iter_custom` closure now produces both criterion's total and the
phase breakdown from the same passes, `phases.rs` is gone, and the previously duplicated harness
lives once in `common/mod.rs`. Items 1, 4 and 7 of [PHASE0_IMPL_ISSUES.md](PHASE0_IMPL_ISSUES.md)
are genuinely resolved rather than papered over.

Also good, and worth keeping:

- Discards are counted by reason and reported with attempt/success counts rather than panicking
  ([key_stream.rs:211](benches/key_stream.rs#L211)) — issue 2 fixed.
- Metadata prefix carries branch, commit, backend, runtime mode, workers and the full criterion
  configuration ([common/mod.rs:340](benches/common/mod.rs#L340)) — issue 7 fixed.
- `runtime_mode: "current_thread_interleaving"` is exactly the label issue 6 asked for.
- Broadcast capacity is `3 × messages_per_key + 1` ([key_stream.rs:59-64](benches/key_stream.rs#L59-L64)),
  so all three send batches fit before the single drain and no cell measures `Lagged` handling.
  Total ring slots stay ~30 k across every distribution, which keeps allocation pressure constant.
- `full_cycle_elements` now counts 3× sends and 3× recvs ([common/mod.rs:157](benches/common/mod.rs#L157))
  — issue 9 fixed.

---

## 1. Blocker — `rc_sender` is not measuring what the decision gate asks

[common/mod.rs:225-235](benches/common/mod.rs#L225-L235),
[src/lib.rs:212-218](src/lib.rs#L212-L218)

`build_rc_sender_cache` resolves **every key's sender before the timed loop starts**, and
`__bench_send_rc_sender` takes the already-resolved handle:

```rust
pub async fn __bench_send_rc_sender(&self, sender: &Rc<broadcast::Sender<V>>, value: V) -> usize {
    sender.send(value).unwrap_or(0)
}
```

So the three variants are not comparable:

| Variant | Map borrow | Hash lookup | Handle clone | Broadcast |
|---|---|---|---|---|
| `guard_held` | yes | yes | — | yes |
| `clone_sender` | yes | yes | `Sender` (4 atomics) | yes |
| `rc_sender` | **no** | **no** | `Rc` (2 non-atomic) | yes |

`rc_sender` will win by roughly the whole lookup — scratch measurement puts `HashMap::get` at
~7.5 ns, versus the ~2-16 ns the entire question is about — for a reason that has nothing to do
with `Rc` versus `Sender`. Decision-gate question 4 cannot be answered from this.

What the plan's `rc_sender` actually is: **the map stores `Rc<broadcast::Sender<V>>`**, and `send`
does borrow → lookup → clone the `Rc` → release guard → broadcast. Only the *handle type* differs
from `clone_sender`; the lookup stays. That requires changing `Streams<K, V>` to
`Rc<RefCell<HashMap<K, Rc<broadcast::Sender<V>>>>>` — a larger change than the current shortcut,
but it is the thing being evaluated.

If a pre-resolved-handle path is worth measuring in its own right, keep it as a **fourth** variant
under a name that says so (`cached_handle`), because it is the per-key handle API discussed
earlier — a different proposal, already ruled out for the web use case where a request handler has
only an id.

## 2. Blocker — `DropValue` has no `Drop` implementation

[common/mod.rs:130-141](benches/common/mod.rs#L130-L141)

```rust
#[derive(Clone)]
pub struct DropValue(pub u64);
```

No `impl Drop`. `needs_drop::<DropValue>()` is therefore `false`, exactly as for `u64`, so the ring
never runs a destructor on eviction. The type is behaviourally identical to `u64` and adds a third
of the total run time for zero information.

It exists to cover the gap `u64` cannot: a value whose destructor runs during
`slot.val = Some(value)` inside `send`, which is where hazard A lives, without `String`'s allocator
noise. Give it a real destructor with an observable side effect so it cannot be optimised away:

```rust
impl Drop for DropValue {
    fn drop(&mut self) {
        DROPS.with(|d| d.set(d.get() + self.0.wrapping_mul(0)) );
    }
}
```

Any body works as long as it is not empty — an empty `Drop` still sets `needs_drop`, but a counter
also lets the bench assert the eviction path was actually exercised, which is worth having.

## 3. High — `send()` has already adopted the fix, ahead of the measurement

[src/lib.rs:178-180](src/lib.rs#L178-L180)

```rust
pub async fn send(&self, key: &K, value: V) -> usize {
    self.__bench_send_clone_sender(key, value).await
}
```

The shipping `send` now releases the guard before broadcasting. That is very likely the right
answer, but three consequences follow that are currently unrecorded:

- Hazard A is **fixed in the library right now**, and neither [CHANGELOG.md](CHANGELOG.md) nor
  [PLAN.md](PLAN.md) says so. PLAN.md still frames the choice as open pending Phase 0 numbers.
- `guard_held` is no longer the default path, so the decision gate's baseline is now reachable only
  through `__bench_send_guard_held`. Fine, but the plan's wording ("today's code") is stale.
- `send_messages_no_receivers` calls the public `send`
  ([common/mod.rs:286](benches/common/mod.rs#L286)), so `send_no_receivers` measures the
  clone_sender path. Harmless — a map miss clones nothing — but it means that phase silently
  changed variant, and it should be stated rather than inferred.

Also worth confirming: does the reentrancy test at [src/lib.rs:835](src/lib.rs#L835) still call
`__bench_send_guard_held`? If so it is testing a path that no longer ships, and there is no test
covering the panic *not* occurring on the path that does.

## 4. High — `build_rc_sender_cache` is inside the measured total but in no phase

[key_stream.rs:80-83](benches/key_stream.rs#L80-L83)

It runs after `Instant::now()` at [key_stream.rs:54](benches/key_stream.rs#L54) but is not wrapped
by any phase span. For `10000key_1msg` that is 10 000 lookups plus 10 000 `Rc` allocations charged
to `full_cycle` and attributed to nothing.

This breaks the self-check the merge was supposed to buy — PLAN.md verification step 6 expects
`full_cycle` total minus the phase sum to stay within ~150 ns of intra-pass `Instant` overhead.
It will be off by milliseconds on the high-key cells.

Fix alongside item 1: if `rc_sender` becomes a real map-backed variant, the cache disappears
entirely. Otherwise hoist the cache build outside the timed region, or give it its own phase.

## 5. Medium — warmup passes are indistinguishable and inflate `max`

Criterion runs the closure during warmup as well, and every iteration is pushed to the accumulator
([key_stream.rs:163-201](benches/key_stream.rs#L163-L201)). PLAN.md accepts this, and it is fine
for `min` and `median` — but the reported `max` is then almost always a warmup pass, making that
column meaningless.

Either drop `max` from the output or replace it with a high percentile (p95) over the sorted
samples, which `min_med_max` is already positioned to compute.

## 6. Medium — `bench_metadata` hardcodes the backend

[common/mod.rs:187-199](benches/common/mod.rs#L187-L199) hardcodes `backend: "local"`,
`runtime_mode: "current_thread_interleaving"`, `workers: 1`.

Correct on this branch, wrong the moment the same harness runs on `bench-sync-rwlock`, and the
error is silent — a mislabelled CSV that still parses. Four series across four branches is exactly
where this bites. Derive it from a `cfg` or from a constant exported by the backend module so it
cannot disagree with the code it is measuring.

## 7. Medium — accumulating cleanup tasks inside one `LocalSet`

[key_stream.rs:157](benches/key_stream.rs#L157) opens one `LocalSet` per `iter_custom` call, and
each pass creates `CREATE_BATCH` = 10 streams, each spawning a cleanup task
([key_stream.rs:58-70](benches/key_stream.rs#L58-L70)). Nine of the ten are dropped immediately,
which aborts but does not necessarily reap them.

With `SamplingMode::Flat` and several iterations per sample, that is tens to hundreds of aborted
tasks queued in a single `LocalSet` before it is dropped. Probably benign, but it makes `create`
partly a measurement of `LocalSet` bookkeeping under growing load rather than of `KeyStream::new`.
Worth checking whether `create` drifts upward with `iters`; if it does, create the streams in their
own short-lived `LocalSet`.

## 8. Low — CSV rows are no longer `parse_criterion.awk`-compatible

Phase rows are `{9 metadata fields},{phase}/{cell},min,med,max,n` and FAILED rows are
`{9 metadata fields},FAILED/{cell},reason,discarded,attempts,n`. Same arity, different meaning per
column, and neither matches the four-column shape
[parse_criterion.awk](bench-results/20260802-181501/parse_criterion.awk) parses.

That is a reasonable trade — PLAN.md chose metadata over awk compatibility — but the awk script
now needs updating or retiring, and the plan still claims the existing tooling works unchanged.
A `kind` column (`phase` / `failed`) as the first field after the prefix would make both row types
unambiguous to parse.

## 9. Low — `#![allow(dead_code)]` now hides real dead code

[common/mod.rs:1](benches/common/mod.rs#L1). It was justified when two targets each used a subset
of the module. With one target it suppresses genuine warnings — `MAX_CLEANUP_YIELDS` and several
helpers may now be unused. Remove it and let the compiler say what the merge orphaned.
