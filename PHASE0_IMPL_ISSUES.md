# Phase 0 Implementation Issues

Review of [benches/phases.rs](benches/phases.rs) and [benches/key_stream.rs](benches/key_stream.rs)
as implemented, against [PLAN.md](PLAN.md). Ordered by severity.

Run-time reduction is tracked in PLAN.md, not here.

---

## 1. High — the two bench targets are ~150 lines of copy-paste

`Mode`, `Distribution`, `BenchValue`, `runtime`, `run_on_localset`, `subscribe_receivers`,
`send_messages`, `recv_all`, `wait_for_cleanup` and the `MODES` / `DISTRIBUTIONS` /
`MAX_CLEANUP_*` constants are duplicated verbatim between the two files.

This is not a style complaint. The whole design depends on the two harnesses producing **identical
cell names over identical workloads** so their CSVs join on the cell — see PLAN.md, "Identical cell
names in both outputs". Two copies guarantee they drift, and the failure is silent: the join still
succeeds, it just compares two different workloads.

Fix: one shared module. Cargo supports `benches/common/mod.rs` with `mod common;` in each target
(no Cargo.toml change), which is the smallest option. A `dev-dependencies` helper crate also works
and gives the shared code its own tests.

## 2. High — cleanup timeout panics and aborts the entire run

[phases.rs:185](benches/phases.rs#L185), [phases.rs:189](benches/phases.rs#L189),
[key_stream.rs:173](benches/key_stream.rs#L173), [key_stream.rs:177](benches/key_stream.rs#L177)

PLAN.md specifies: discard the sample and emit a visible `FAILED` row so bad samples are
visible. The implementation panics instead, so one slow pass destroys a multi-minute run and
loses every cell measured before it.

Fix: return `Option<[f64; 6]>` from `run_pass`, drop the sample on timeout, and print a `FAILED`
row naming the cell. Report the discard count per cell — a cell that dropped 20 of 50 samples is
not comparable to one that dropped none, and a median computed over the survivors hides that.

## 3. High — the hazard-A decision has no data path

Only `guard_held` exists. The phase is honestly labelled
`send_with_receivers/guard_held` ([phases.rs:261](benches/phases.rs#L261)), but decision-gate
question 4 in PLAN.md compares `guard_held` / `clone_sender` / `rc_sender`, and two of the three
are unimplemented. As it stands Phase 1 cannot answer which fix to ship.

Fix: `send` is a two-line function. Either add the two variants to the lib behind a bench-only
feature, or reimplement all three in the shared bench module against the map directly. The latter
keeps the lib clean but risks measuring something subtly different from what ships — prefer the
former.

## 4. Medium — inconsistent handling of the cleanup bound between targets

In `phases.rs` a slow cleanup panics. In `key_stream.rs` the same wait sits **inside** the
measured span of `run_full_cycle_once` ([key_stream.rs:191](benches/key_stream.rs#L191)), so a
stall short of 1 s is silently recorded as a legitimate sample and inflates the median, while a
stall over 1 s panics. Two different behaviours either side of an arbitrary threshold.

Fix: same discard-and-report policy in both, with the same bound.

## 5. Medium — `create` is timed as a single operation

[phases.rs:200-203](benches/phases.rs#L200-L203)

`out[0] = per_op_ns(create_start.elapsed(), 1)` wraps one `KeyStream::new` in an `Instant::now()`
pair. The pair costs ~20-25 ns around an operation of maybe 100-300 ns, so that column carries
~10% clock error — exactly what PLAN.md says to avoid ("phases are timed rather than operations").

Fix: construct 100 streams in the phase and divide by 100.

## 6. Medium — `sender_tasks` means something different on each backend, and nothing labels it

On `local`, every sender task runs on one thread through `join_all`, so `16x1` and `4x4` measure
*future interleaving*, not lock contention — there is no parallelism to contend for. On `shared`
with a multi-thread runtime, the same modes measure real contention. Same cell name, two different
experiments.

The matrix keeps all four modes on both backends (deliberate — see PLAN.md), so the fix is
labelling, not deletion: mark the local rows as concurrency-without-parallelism in the output, or
the natural reading of "`local` wins `16x1` by N%" is exactly backwards — it wins because it never
paid for contention it also cannot exploit.

Expect `1x1` and `16x1` to be near-identical on `local`. If they are not, that difference is
`join_all` polling overhead and is worth understanding before trusting the shared-side numbers.

## 7. Medium — no metadata columns

Neither target emits branch, commit, backend, runtime mode, worker count, `W`, or `P`. PLAN.md
requires these prefixed as leading columns. With four series compared across four branches, an
unlabelled CSV is the single likeliest way to lose a whole run.

## 8. Low — `String` sends are roughly half allocator

`V::from_index` is `format!("v{index}")` ([phases.rs:86](benches/phases.rs#L86)) and is called
inside the timed send loop ([phases.rs:156](benches/phases.rs#L156)).

Defensible — a real caller does allocate — but it must be labelled, because the `clone_sender`
delta being hunted is a handful of nanoseconds and will be buried in malloc noise. Consider a
third value type with a non-trivial `Drop` but no allocation (e.g. a struct wrapping a `u64` with
a manual `Drop`), which exercises the eviction-drop path that `u64` cannot while staying cheap.

## 9. Low — `full_cycle` throughput is under-counted

[key_stream.rs:205](benches/key_stream.rs#L205) sets
`Throughput::Elements(dist.total_sends())`, but the measured cycle performs `total_sends` sends
with receivers, another `total_sends` without, plus `total_sends × receivers_per_key` receives,
plus subscribes and drops. Criterion's reported elements/sec is therefore wrong by a factor that
varies with mode — which makes throughput look mode-dependent for reasons unrelated to the code
under test.

Fix: count every element the cycle processes, or drop `throughput` and compare wall time only.

## 10. Low — `recv_all` concentrates value-drop cost on the last receiver

[phases.rs:164-176](benches/phases.rs#L164-L176) drains receiver-by-receiver rather than
round-robin. Tokio clears a ring slot when its last reader releases it, so every `V::drop` in a
multi-receiver mode is attributed to whichever receiver happens to drain last. Harmless for
totals, but it means per-receiver timings inside `recv` are not comparable in `1x16` and `4x4`.
Worth a comment if the draining order is deliberate.

---

## Reviewer Disagreements (No Decisions)

Only points I disagree with are listed below. This section is informational and intentionally
does not make implementation choices.

1. Item 3 (`guard_held` / `clone_sender` / `rc_sender` data path):
- I agree the missing variants block the hazard-A decision.
- I do **not** agree with the strong preference to add bench-only feature gates in the library
	before deciding. Measuring all three variants in shared bench code can be valid for Phase 0 as
	long as the benchmark path stays behavior-identical and the shipping path is selected only after
	numbers are in.

2. Item 5 (`create` as single op):
- I agree one-op timing is noisy.
- I do **not** agree that a hard-coded `100` is necessarily the right constant. The requirement is
	amortization, not a specific multiplier. Any fixed loop count that pushes clock overhead below
	noise is acceptable.

3. Item 10 (`recv_all` order):
- For current outputs (phase totals only), this is lower impact than stated because no
	per-receiver breakdown is reported.
- If per-receiver stats are later added, the concern becomes material and should be addressed then.
