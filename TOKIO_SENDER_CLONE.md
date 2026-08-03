# `broadcast::Sender::clone` is ~2-4x an `Arc<Sender>` clone — notes for a tokio issue

Parked for later investigation. Everything below is reproducible from the
self-contained program at the bottom; drop it into a fresh
`cargo new --bin` with `tokio = { version = "1", features = ["sync"] }`.

## The observation

While benchmarking `key-stream`, cloning a `broadcast::Sender` out of a map
before releasing a guard cost **+5.8 ns/send on arm64**, whereas storing
`Rc<Sender>`/`Arc<Sender>` in the map and cloning *that* cost ~2 ns — despite
addressing the same channel and doing strictly more pointer work.

## Mechanism

`Sender::clone` is `Arc::clone` **plus** a second counter, and `Sender::drop`
decrements it with `AcqRel` on *every* drop:

```rust
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Sender<T> {
        let shared = self.shared.clone();       // Arc strong, Relaxed
        shared.num_tx.fetch_add(1, Relaxed);    // second counter
        Sender { shared }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        if 1 == self.shared.num_tx.fetch_sub(1, AcqRel) {   // AcqRel every time
            self.close_channel();
        }
    }
}
```

`std::sync::Arc` solves the same problem more cheaply — `Release` on every
decrement, `Acquire` fence only in the branch where the count hit zero:

```rust
if self.inner().strong.fetch_sub(1, Release) != 1 { return; }
acquire!(self.inner().strong);   // fence(Acquire), last drop only
unsafe { self.drop_slow(); }
```

### Why `num_tx` can't just be the `Arc` strong count

Both halves of the channel own the same allocation:

```rust
pub struct Sender<T>     { shared: Arc<Shared<T>> }
pub struct Receiver<T>   { shared: Arc<Shared<T>> }   // strong, not weak
pub struct WeakSender<T> { shared: Arc<Shared<T>> }   // also strong
```

So `strong_count` = senders + receivers + weak senders. But the channel must
call `close_channel()` — waking every receiver with `RecvError::Closed` — when
the last **Sender** drops, while receivers are still alive holding their own
`Arc`. `WeakSender` sharpens it: it holds a *strong* `Arc` (the allocation must
stay upgradeable) while deliberately not counting as a producer. That's
"keeps the allocation alive but isn't a sender", which `Arc`'s own strong/weak
split cannot express.

**The counter is load-bearing. Its ordering may not be.**

## Measured

Apple M1 Pro, aarch64, min of 5 × 30M iterations:

| | ns | what it is |
|---|---:|---|
| `Arc<u64>::clone` + drop | 9.67 | one pair, Relaxed/Release |
| `Arc<Sender>::clone` + drop | 9.63 | identical — it *is* just an Arc |
| `broadcast::Sender::clone` + drop | **16.44** | two pairs, second `AcqRel` |

Isolating which property costs:

| | ns |
|---|---:|
| 1 pair, Relaxed / Relaxed | 4.28 |
| 1 pair, Relaxed / **Release** | 9.71 |
| 1 pair, Relaxed / **AcqRel** | 13.28 |
| 2 pairs, same cache line, Relaxed/Release | 11.05 |
| 2 pairs, **separate** cache lines, Relaxed/Release | 11.05 |
| 2 pairs, separate lines, 2nd sub **AcqRel** | **16.43** |

Two conclusions:

- **Cache-line separation costs nothing** (11.05 either way). I expected this to
  matter and it does not.
- **The last row models `Sender::clone` to within 0.01 ns** (16.43 vs 16.44).
  The cost is the `AcqRel`, not the extra counter as such.

### It is architecture-dependent

In-context deltas from the key-stream matrix, `clone_sender` vs a baseline that
holds the guard instead:

| | arm64 (M1 Pro) | x86_64 runner A | x86_64 runner B |
|---|---:|---:|---:|
| marginal cost, 1st atomic pair | +0.12 ns | — | +10.3 ns |
| `clone_sender` total | +5.8 ns | +5.3 ns | +21.7 ns |

x86 has **no cheap atomic RMW** — every `lock` prefix is a full barrier, so
ordering is irrelevant there and count is everything. ARM has cheap
Relaxed/Release RMWs, so only the `AcqRel` shows up.

⚠️ The two x86 columns are different GitHub Actions runners and the CPU models
were not recorded. `lock`-op latency varies several-fold across that pool, so
**the x86 numbers are not yet attributable**. Re-run the program below on a
known CPU before quoting them.

## Proposed fix

Apply `Arc`'s own pattern:

```rust
impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        if self.shared.num_tx.fetch_sub(1, Release) == 1 {
            std::sync::atomic::fence(Acquire);
            self.close_channel();
        }
    }
}
```

Soundness is `Arc`'s argument verbatim: every decrement releases, and the last
one acquires before touching `Shared`. Arguably even the fence is redundant,
since `close_channel` immediately takes `tail.lock()`, which is itself an
acquire. `WeakSender::drop` has the same unconditional `AcqRel` on `num_weak_tx`.

**Expected to be an ARM-only win.** On x86 `fetch_sub(Release)` and
`fetch_sub(AcqRel)` both compile to `lock xadd` — same instruction, so no
saving. Frame any upstream issue that way rather than claiming a general
speedup.

## Prior art check

- tokio's only broadcast benchmark, [`benches/sync_broadcast.rs`](https://github.com/tokio-rs/tokio/blob/master/benches/sync_broadcast.rs),
  is a single `bench_contention` test — N receivers waking on a send. The sender
  is cloned during *setup*, never inside a timed loop, so this has never been
  measured upstream.
- Nothing else in tokio's bench suite times channel-handle clone or drop.
- No existing issue or PR found on the ordering. The known broadcast perf issue,
  [#5923](https://github.com/tokio-rs/tokio/issues/5923), is about `send` latency
  scaling with receiver count — unrelated.

## Open questions

1. Does the win survive on x86? Predicted no. Measure on a known CPU.
2. Is the `Acquire` fence needed at all, given `close_channel` takes a mutex?
3. Same treatment for `num_weak_tx` in `WeakSender::drop`?
4. Would tokio accept an ARM-targeted micro-optimisation, or is the ordering
   deliberately conservative for loom's benefit? Check whether the loom model
   distinguishes these.

## The program

```rust
use std::hint::black_box;
use std::rc::Rc;
use std::sync::atomic::{fence, AtomicUsize, Ordering::*};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::broadcast;

const ITERS: u64 = 30_000_000;
const REPEATS: usize = 5;

// min, not mean: noise only ever adds time.
fn bench(label: &str, mut f: impl FnMut()) -> f64 {
    let mut best = f64::MAX;
    for _ in 0..REPEATS {
        let t = Instant::now();
        for _ in 0..ITERS { f(); }
        best = best.min(t.elapsed().as_nanos() as f64 / ITERS as f64);
    }
    println!("  {label:<52} {best:>7.2} ns");
    best
}

#[repr(align(128))]
struct Padded(AtomicUsize);

fn cpu_model() -> String {
    #[cfg(target_os = "linux")]
    if let Ok(s) = std::fs::read_to_string("/proc/cpuinfo") {
        for line in s.lines() {
            if line.starts_with("model name") {
                if let Some((_, v)) = line.split_once(':') { return v.trim().into(); }
            }
        }
    }
    #[cfg(target_os = "macos")]
    if let Ok(o) = std::process::Command::new("sysctl")
        .args(["-n", "machdep.cpu.brand_string"]).output()
    {
        let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
        if !s.is_empty() { return s; }
    }
    "unknown".into()
}

fn main() {
    println!("arch = {}\ncpu  = {}\n{ITERS} iters, min of {REPEATS}\n",
             std::env::consts::ARCH, cpu_model());

    let (tx, _rx) = broadcast::channel::<u64>(16);
    let arc_tx = Arc::new(tx.clone());
    let rc_tx = Rc::new(tx.clone());
    let arc_plain = Arc::new(0u64);

    println!("[1] real types — clone + drop");
    let t_rc     = bench("Rc<Sender>::clone",        || { black_box(rc_tx.clone()); });
    let t_plain  = bench("Arc<u64>::clone",          || { black_box(arc_plain.clone()); });
    let t_arc    = bench("Arc<Sender>::clone",       || { black_box(arc_tx.clone()); });
    let t_sender = bench("broadcast::Sender::clone", || { black_box(tx.clone()); });
    println!("  -> Sender costs {:+.2} ns over Arc<Sender> on the same channel\n",
             t_sender - t_arc);

    let same = [AtomicUsize::new(1), AtomicUsize::new(1)];
    let a = Padded(AtomicUsize::new(1));
    let b = Padded(AtomicUsize::new(1));

    println!("[2] one pair — is it the ORDERING?");
    let relaxed = bench("Relaxed add + Relaxed sub", || {
        same[0].fetch_add(1, Relaxed); black_box(same[0].fetch_sub(1, Relaxed)); });
    let release = bench("Relaxed add + Release sub   (= Arc)", || {
        same[0].fetch_add(1, Relaxed); black_box(same[0].fetch_sub(1, Release)); });
    let acqrel  = bench("Relaxed add + AcqRel sub    (= num_tx)", || {
        same[0].fetch_add(1, Relaxed); black_box(same[0].fetch_sub(1, AcqRel)); });
    println!("  -> Release {:+.2} over Relaxed; AcqRel {:+.2} over Release\n",
             release - relaxed, acqrel - release);

    println!("[3] two pairs — is it the COUNT?");
    let two = bench("2 pairs, both Relaxed/Release", || {
        same[0].fetch_add(1, Relaxed); same[1].fetch_add(1, Relaxed);
        same[1].fetch_sub(1, Release); black_box(same[0].fetch_sub(1, Release)); });
    let model = bench("2 pairs, 2nd sub AcqRel  (models Sender)", || {
        same[0].fetch_add(1, Relaxed); same[1].fetch_add(1, Relaxed);
        same[1].fetch_sub(1, AcqRel);  black_box(same[0].fetch_sub(1, Release)); });
    println!("  -> 2nd pair {:+.2}; model is {:+.2} from real Sender::clone\n",
             two - release, model - t_sender);

    println!("[4] cache line — must the counters share one?");
    let diff = bench("2 pairs, separate cache lines", || {
        a.0.fetch_add(1, Relaxed); b.0.fetch_add(1, Relaxed);
        b.0.fetch_sub(1, Release); black_box(a.0.fetch_sub(1, Release)); });
    println!("  -> separation costs {:+.2} ns\n", diff - two);

    println!("[5] the proposed fix");
    let now = bench("current:  AcqRel sub every drop", || {
        same[0].fetch_add(1, Relaxed); black_box(same[0].fetch_sub(1, AcqRel)); });
    let fixed = bench("proposed: Release sub, Acquire fence if last", || {
        same[0].fetch_add(1, Relaxed);
        if black_box(same[0].fetch_sub(1, Release)) == 1 { fence(Acquire); }
    });
    println!("  -> saves {:+.2} ns per clone/drop cycle here\n", fixed - now);

    println!("Rc {t_rc:.2} | Arc<u64> {t_plain:.2} | Arc<Sender> {t_arc:.2} | Sender {t_sender:.2}");
    black_box((rc_tx, arc_tx, arc_plain, tx));
}
```
