# Local full-matrix run — `bench-variants` OFF (production shape)

Machine: Apple Silicon (arm64), macOS. Quiet, no other load.
Command: `cargo bench --bench key_stream -- --noplot`
Coverage: 60 benchmark IDs (4 modes × 5 distributions × 3 value types), 420 phase
rows, **0 discarded samples**.

`phases.csv` columns:
`branch,commit,backend,runtime_mode,workers,warmup_ms,measurement_ms,samples,sampling,kind,phase,cell,min_ns,med_ns,p95_ns,n`

Statistic to read is `min_ns` — noise only ever adds time, and run-to-run spread
on `min` was 0.05–0.5 ns across three repeats of a single cell.

## Results

**`clone_sender` penalty vs `guard_held`: +4.1 to +7.6 ns in all 60 cells.**
Never free, never large. `String` cells sit at the top of that range, `u64` and
`DropValue` at the bottom. Mode has no effect, as expected on a single-threaded
backend. This is tokio's `num_tx` increment and nothing else.

**`subscribe` and `drop` are dominated by broadcast ring alloc/free, not by map
work** — the ring is `messages_per_key + 1` slots, so per-receiver cost scales
inversely with key count:

| distribution | subscribe | drop |
|---|---:|---:|
| `1key_10000msg` | 27 959 ns | 261 458 ns |
| `100key_100msg` | 377 ns | 2 196 ns |
| `10000key_1msg` | 158 ns | 183 ns |

`drop` also divides by receivers-per-key: at `1key_10000msg` it is 261 458 ns in
`1x1` and 16 372 in `1x16` — one ring teardown split 16 ways, not a real
difference.

**Consequence for Phase 1:** the decision gate must read `10000key_1msg` and
`1000key_10msg` only. In the low-key cells, reclamation is buried under ring
teardown by three orders of magnitude and no change to cleanup strategy could
show there.

## Known distortion in this data

`send_no_receivers` constructs the value before the failed lookup, so for
`V = String` it is ~85% allocation (69–74 ns) rather than the ~11–15 ns map miss
that `u64` shows. Read the `u64` and `DropValue` rows for that phase.
