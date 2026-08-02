# Make CI benchmark runs interpretable

## Context

The first CI benchmark run produced numbers that cannot be interpreted, and one
pair of them actively contradicts the other. Every phase row was labelled:

```
HEAD,025c6c1-dirty,local,current_thread_interleaving,1,500,3000,20,flat,phase,...
```

Three defects in that prefix, and one in the job structure:

**`branch=HEAD`.** `actions/checkout` leaves a detached HEAD, so
`git rev-parse --abbrev-ref HEAD` returns the literal string `HEAD`. Every CI row
is unattributable to a branch.

**`-dirty` on a pristine checkout.** The workflow's own `tee bench-output.txt`
and `machine.txt` landed in the repo root, and `git status --porcelain` counts
untracked files. So every run self-reported dirty — inverting the meaning of a
marker whose only job is to warn that measured code does not match the recorded
commit.

**No CPU identity in the CSV.** `lscpu` went to a sidecar file. Merged rows
cannot be told apart, and arch alone would not have been enough here: both jobs
were x86_64.

**The `strategy.matrix` split the two feature configs across two runners.** This
is the one that did real damage. `clone_sender` measured +5.3 ns in one job and
+21.7 ns in the other, for the same cell.

### Why we know it was the runners, not the code

`send_no_receivers` is byte-identical in both builds — a map miss touching
neither handles nor variants. Locally, on a quiet machine, the two configs
produce **11.11 ns and 11.12 ns**. The two CI jobs produced **23.0 ns and
12.2 ns**, and `recv` moved 27% in the opposite direction. No build difference
can do that; the jobs ran on different CPUs. `lock`-op latency varies several-fold
across the models in GitHub's pool, which is exactly the quantity under study.

Consequence: both x86 figures are real for their own CPU, and neither can be
compared to the other or to the arm64 baseline until the CPU is recorded.

## Changes

### 1. `.github/workflows/bench.yaml` — already applied

- **Matrix collapsed into one job**, both configs run sequentially so they share
  a CPU. Doubles wall time; comparability is worth more than parallelism here.
  Production-shape config runs first, so it sees the *less* warmed machine —
  biasing against the config that feeds Phase 1's decision gate, not for it.
- **Artifacts write to `$RUNNER_TEMP/bench`**, outside the checkout, so the tree
  stays clean and `-dirty` regains its meaning.
- Coverage check per config (60 IDs; 420 rows off / 540 on; discards warn).
- Outstanding nit: the upload step's `${{ env.OUT }}` raises a linter warning
  ("Context access might be invalid"). It resolves correctly at runtime, but
  using `${{ runner.temp }}/bench` directly avoids the warning.

### 2. `benches/common/mod.rs` — `bench_metadata()` and `csv_prefix()`

Prefer CI-provided identity, fall back to git:

- `branch`: `GITHUB_HEAD_REF` (PRs), else `GITHUB_REF_NAME`, else
  `git rev-parse --abbrev-ref HEAD`.
- `commit`: `GITHUB_SHA` (shortened), else `git rev-parse --short HEAD`.
- dirty check: `git status --porcelain --untracked-files=no`, so build outputs
  and untracked scratch never mark a run dirty. Only tracked-file modifications
  do — which is the condition worth warning about.

Two new prefix columns, before `backend`:

- `arch`: `std::env::consts::ARCH`.
- `cpu`: model string — `/proc/cpuinfo` `model name` on Linux,
  `sysctl -n machdep.cpu.brand_string` on macOS, `"unknown"` elsewhere.
  **Must be sanitised**: commas and newlines replaced, since the output is naive
  CSV with no quoting. AMD and Intel model strings contain `@`, parentheses and
  sometimes commas.

Reuse the existing `git_value` helper rather than adding a second subprocess
path.

### 3. Documentation

- `bench-results/local-full/README.md` and `local-full-variants/README.md`:
  update the documented column list, and record the arm64 CPU so the existing
  baselines stay comparable to future runs.
- `PLAN.md`: the "Measured in-context" table currently presents arm64 numbers as
  settled. Add that per-atomic-pair cost is architecture- and CPU-dependent
  (~0.1 ns marginal for the first pair on arm64 vs 2.6-10.5 ns on x86 depending
  on model), and that Q4b cannot be answered until a known CPU is pinned down.

## Verification

1. `cargo test`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`
   in **both** feature configurations.
2. `cargo bench --bench key_stream -- --noplot '1x1/1key_10000msg/u64'` and
   confirm the prefix now carries a real branch name, a clean (non-`-dirty`)
   commit on a clean tree, `aarch64`, and the local CPU model with no stray
   commas breaking column alignment.
3. Deliberately dirty a tracked file, re-run one cell, confirm `-dirty` returns.
   Then create an untracked file and confirm it does **not**.
4. Confirm column count is stable: `awk -F, '{print NF}' phases.csv | sort -u`
   should print exactly one number.
5. On CI: both configs appear in one job with one `machine.txt`, and the `cpu`
   column matches `lscpu`.

## Follow-up, not in scope here

Ask for the two `machine.txt` artifacts from the completed run. Naming the two
CPUs turns the contradiction into a result — "an atomic RMW pair costs X ns on
model A and Y ns on model B" is a genuinely useful finding, and it decides
whether the shared backend's `Arc` handle is free (as on arm64) or expensive
(as on one of the two runners).
