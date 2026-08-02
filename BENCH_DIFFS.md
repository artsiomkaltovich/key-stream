# Benchmark Plan Notes

Scope: keep benchmark structure aligned between `bench` and `bench-sync-rwlock` while preserving
intentional runtime differences.

## Current Structure

1. `benches/key_stream.rs` now contains one Criterion group: `full_cycle`.
2. `benches/phases.rs` is a custom harness (`harness = false`) that emits per-phase CSV rows.
3. Both targets use the same matrix dimensions:
- mode: `1x1`, `16x1`, `1x16`, `4x4`
- distribution: `1key_10000msg`, `100key_100msg`, `10000key_1msg`
- value: `u64`, `String`

## Rules Kept

1. Creation cost is measured.
2. Cleanup/reclamation cost is measured.
3. Full lifecycle remains the primary end-to-end check (`full_cycle`).
4. Local vs non-local runtime differences are intentional and must stay explicit.

## Phase Harness Output

`benches/phases.rs` prints parse-compatible CSV rows in four-column form:

`name,min_ns,median_ns,max_ns`

Phase names include:
- `create`
- `subscribe`
- `send_with_receivers/guard_held`
- `send_no_receivers`
- `recv`
- `drop`

## Validation

- `cargo check --benches`
