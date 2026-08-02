# Local full-matrix run — `bench-variants` ON (handle comparison)

Machine: Apple Silicon (arm64), macOS. Quiet, no other load.
Command: `cargo bench --features bench-variants --bench key_stream -- --noplot`
Coverage: 60 benchmark IDs, 540 phase rows (9 phases: 4 send variants), **0 discards**.

Statistic is `min_ns`. Read the send-variant rows only — with this feature on,
the map entry carries three handles, which inflates `subscribe` by ~18% and
`drop` by ~16% per key. For those phases use the sibling `local-full/` run.

## Result: all 60 cells, delta vs `guard_held`

| variant | mean | median | range |
|---|---:|---:|---|
| `rc_sender` | +1.96 ns | +1.98 | +1.01 … +5.21 |
| `arc_sender` | +2.08 ns | +2.06 | +1.00 … +4.30 |
| `clone_sender` | **+5.77 ns** | +5.84 | +4.16 … +6.37 |

**`rc` vs `arc` is a coin flip.** Head-to-head over 60 cells: mean +0.12 ns,
median +0.10, range −0.91 … +0.80, with `arc` ahead in 18 of 60. There is no
measurable penalty for the atomic refcount at this scale.

## Correction to an earlier single-cell reading

An earlier run of `1x1/1key_10000msg/u64` alone put `rc_sender` at +0.03 ns and
`arc_sender` at −0.23 ns, and was reported as "the handle variants are free".
That was an artifact of the hottest possible map — one entry, permanently
cache-resident. Across the full sweep both cost **~2 ns**. The direction held;
the magnitude did not. Single-cell results were wrong twice in this exercise
(see also the fixed-variant-order bias); the matrix exists for this reason.

## What still stands

`clone_sender` is ~3x the cost of either wrapped handle, in every cell, for
every value type. The gap is tokio's `num_tx` increment on top of the `Arc`
that `Sender::clone` already does — not the `Arc` itself.

## Consequences

- The wrapped-handle fix is cheap but not free: ~2 ns on a ~45-100 ns send.
- Choosing `Arc` for the shared backend costs nothing relative to `Rc`, so the
  handle question does not argue for or against the local/shared split.
- Still unmeasured: contention. Every number here is single-threaded, so
  `16x1` and `4x4` show scheduling interleave, not lock contention. The case
  where `guard_held` blocks writers cannot appear until a real shared backend
  runs on a multi-thread runtime.
