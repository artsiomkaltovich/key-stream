# Benchmark Branch Diff And Fix Plan

Scope: compare branch `bench` and branch `bench-sync-rwlock` for `benches/key_stream.rs`.

## Required Rules

1. Benchmarks should create one stream/sender per measured sample, not per inner iteration.
2. Exception: benches that are explicitly multi-sender may recreate sender/stream per inner iteration if that is the intended workload.
3. Creation cost is part of the measurement.
4. Cleanup cost is part of the measurement.
5. Full cycle benches are preferred for end-to-end behavior.
6. Local vs non-local task execution difference is intentional for comparison.

## Differences Found (Before Fix)

1. Timing model mismatch
- `bench`: mixed `to_async(...).iter(...)` and `iter_custom(...)`.
- `bench-sync-rwlock`: mostly `iter_custom(...)`.
- Effect: different iteration boundaries and timed scope.

2. State reuse mismatch in `subscribe_existing_key`
- `bench`: sender/stream reused across benchmark loop.
- `bench-sync-rwlock`: sender/stream recreated in measured path.
- Effect: branch `bench` measured growing shared state.

3. Cleanup timing mismatch
- Both branches had cases where cleanup reaping (`yield_now`) was outside measured duration.
- Effect: measured duration excluded some cleanup cost.

4. Creation timing mismatch
- Some benches created stream/sender once per sample, then timed only send loop.
- Effect: creation cost excluded from measurement.

5. Full-cycle mismatch
- Some benches measured only hot path operations and not full create-send-receive-cleanup cycle.

6. Executor model mismatch (intentional)
- `bench`: local execution path (`LocalSet` / `spawn_local`) in local-specific cases.
- `bench-sync-rwlock`: non-local path (`spawn`).
- This difference is intentional to test whether local scheduling helps.

## Fix Applied

For both branches, apply the same benchmark structure except where local vs non-local is intentionally different:

1. Use `iter_custom` for explicit control over measured cycle.
2. For non-multi-sender benches, measure one sample as:
- start timer,
- create stream/sender/receivers once,
- run `iters` operations,
- drop handles,
- `yield_now` for cleanup reaping,
- then record elapsed time.
3. For multi-sender benches, keep per-iteration recreate behavior if that behavior is the intended test.
4. Keep local-vs-non-local executor difference only where that comparison is intended.

## Expected Remaining Cross-Branch Difference

Only executor model differences needed for local vs non-local comparison should remain.

## Validation

- `cargo check --benches`
