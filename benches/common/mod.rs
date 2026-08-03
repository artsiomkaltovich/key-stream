use futures::future::join_all;
use key_stream::{KeyReceiver, KeySender, KeyStream, Value};
use std::cell::Cell;
use std::future::Future;
use std::process::Command;
use std::time::{Duration, Instant};
use tokio::runtime::{Builder, Runtime};
use tokio::task::LocalSet;

#[derive(Clone, Copy)]
pub struct Mode {
    pub name: &'static str,
    pub sender_tasks: usize,
    pub receivers_per_key: usize,
}

#[derive(Clone, Copy)]
pub struct Distribution {
    pub name: &'static str,
    pub keys: usize,
    pub messages_per_key: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SendWithReceiversVariant {
    /// Guard held across the broadcast. Unsafe (hazard A: a re-entrant value
    /// destructor panics); carried only as the zero-cost baseline.
    GuardHeld,
    RcSender,
    ArcSender,
}

#[derive(Clone)]
pub struct BenchMetadata {
    pub branch: String,
    pub commit: String,
    pub arch: String,
    pub cpu: String,
    pub backend: String,
    pub runtime_mode: String,
    pub workers: usize,
    pub criterion_warmup_ms: u64,
    pub criterion_measurement_ms: u64,
    pub criterion_sample_size: usize,
    pub criterion_sampling_mode: String,
}

pub const MODES: [Mode; 4] = [
    Mode {
        name: "1x1",
        sender_tasks: 1,
        receivers_per_key: 1,
    },
    Mode {
        name: "16x1",
        sender_tasks: 16,
        receivers_per_key: 1,
    },
    Mode {
        name: "1x16",
        sender_tasks: 1,
        receivers_per_key: 16,
    },
    Mode {
        name: "4x4",
        sender_tasks: 4,
        receivers_per_key: 4,
    },
];

pub const DISTRIBUTIONS: [Distribution; 5] = [
    Distribution {
        name: "1key_10000msg",
        keys: 1,
        messages_per_key: 10_000,
    },
    Distribution {
        name: "10key_1000msg",
        keys: 10,
        messages_per_key: 1_000,
    },
    Distribution {
        name: "100key_100msg",
        keys: 100,
        messages_per_key: 100,
    },
    Distribution {
        name: "1000key_10msg",
        keys: 1_000,
        messages_per_key: 10,
    },
    Distribution {
        name: "10000key_1msg",
        keys: 10_000,
        messages_per_key: 1,
    },
];

pub const CRITERION_WARMUP_MS: u64 = 500;
pub const CRITERION_MEASUREMENT_MS: u64 = 3_000;
pub const CRITERION_SAMPLE_SIZE: usize = 20;
pub const CREATE_BATCH: usize = 10;
pub const MAX_CLEANUP_YIELDS: usize = 10_000;
pub const MAX_CLEANUP_WAIT: Duration = Duration::from_secs(1);

thread_local! {
    static DROPVALUE_DROPS: Cell<u64> = const { Cell::new(0) };
}

pub trait BenchValue: Value + Clone + 'static {
    fn from_index(index: u64) -> Self;
    fn label() -> &'static str;
}

impl BenchValue for u64 {
    fn from_index(index: u64) -> Self {
        index
    }

    fn label() -> &'static str {
        "u64"
    }
}

impl BenchValue for String {
    fn from_index(index: u64) -> Self {
        format!("v{index}")
    }

    fn label() -> &'static str {
        "String"
    }
}

#[derive(Clone)]
pub struct DropValue(pub u64);

impl Drop for DropValue {
    fn drop(&mut self) {
        std::hint::black_box(self.0);
        DROPVALUE_DROPS.with(|count| count.set(count.get().wrapping_add(1)));
    }
}

impl BenchValue for DropValue {
    fn from_index(index: u64) -> Self {
        Self(index)
    }

    fn label() -> &'static str {
        "DropValue"
    }
}

impl Distribution {
    pub fn total_sends(self) -> usize {
        self.keys * self.messages_per_key
    }

    pub fn recv_messages(self, mode: Mode) -> usize {
        self.total_sends() * mode.receivers_per_key
    }

    pub fn receivers(self, mode: Mode) -> usize {
        self.keys * mode.receivers_per_key
    }
}

pub fn full_cycle_elements(mode: Mode, dist: Distribution) -> u64 {
    let sends_with = (dist.total_sends() * send_variants().len()) as u64;
    let sends_none = dist.total_sends() as u64;
    let recvs = (dist.recv_messages(mode) * send_variants().len()) as u64;
    let subs_and_drops = (dist.receivers(mode) * 2) as u64;
    sends_with + sends_none + recvs + subs_and_drops
}

const SEND_VARIANTS: [SendWithReceiversVariant; 3] = [
    SendWithReceiversVariant::GuardHeld,
    SendWithReceiversVariant::RcSender,
    SendWithReceiversVariant::ArcSender,
];

pub fn send_variants() -> &'static [SendWithReceiversVariant] {
    &SEND_VARIANTS
}

pub fn send_variant_label(variant: SendWithReceiversVariant) -> &'static str {
    match variant {
        SendWithReceiversVariant::GuardHeld => "guard_held",
        SendWithReceiversVariant::RcSender => "rc_sender",
        SendWithReceiversVariant::ArcSender => "arc_sender",
    }
}

pub struct BenchRuntime {
    pub rt: Runtime,
    pub mode: &'static str,
    pub workers: usize,
}

pub fn runtime() -> BenchRuntime {
    let rt = Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");
    BenchRuntime {
        rt,
        mode: "current_thread_interleaving",
        workers: 1,
    }
}

pub async fn run_on_localset<F, T>(fut: F) -> T
where
    F: Future<Output = T>,
{
    LocalSet::new().run_until(fut).await
}

pub fn bench_metadata(runtime_mode: &str, workers: usize) -> BenchMetadata {
    let git_commit = git_value(["rev-parse", "--short", "HEAD"]);
    let short_len = git_commit.len().max(7);
    let mut commit = env_non_empty("GITHUB_SHA")
        .map(|sha| sha.chars().take(short_len).collect::<String>())
        .unwrap_or(git_commit);
    if git_is_dirty() {
        commit.push_str("-dirty");
    }
    BenchMetadata {
        branch: env_non_empty("GITHUB_HEAD_REF")
            .or_else(|| env_non_empty("GITHUB_REF_NAME"))
            .unwrap_or_else(|| git_value(["rev-parse", "--abbrev-ref", "HEAD"])),
        commit,
        arch: std::env::consts::ARCH.to_string(),
        cpu: cpu_model(),
        backend: key_stream::__BENCH_BACKEND.to_string(),
        runtime_mode: runtime_mode.to_string(),
        workers,
        criterion_warmup_ms: CRITERION_WARMUP_MS,
        criterion_measurement_ms: CRITERION_MEASUREMENT_MS,
        criterion_sample_size: CRITERION_SAMPLE_SIZE,
        criterion_sampling_mode: "flat".to_string(),
    }
}

fn git_value<const N: usize>(args: [&str; N]) -> String {
    command_value("git", &args)
}

fn command_value(command: &str, args: &[&str]) -> String {
    let output = Command::new(command).args(args).output();
    match output {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).trim().to_owned(),
        _ => "unknown".to_string(),
    }
}

fn git_is_dirty() -> bool {
    let output = Command::new("git")
        .args(["status", "--porcelain", "--untracked-files=no"])
        .output();
    match output {
        Ok(out) if out.status.success() => !out.stdout.is_empty(),
        _ => false,
    }
}

fn env_non_empty(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

#[cfg(target_os = "linux")]
fn cpu_model() -> String {
    if let Ok(cpuinfo) = std::fs::read_to_string("/proc/cpuinfo") {
        for line in cpuinfo.lines() {
            if let Some((_, value)) = line.split_once(':')
                && line.starts_with("model name")
            {
                return sanitize_csv_field(value);
            }
        }
    }
    sanitize_csv_field("unknown")
}

#[cfg(target_os = "macos")]
fn cpu_model() -> String {
    sanitize_csv_field(&command_value(
        "sysctl",
        &["-n", "machdep.cpu.brand_string"],
    ))
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn cpu_model() -> String {
    sanitize_csv_field("unknown")
}

fn sanitize_csv_field(raw: &str) -> String {
    let cleaned = raw.replace(',', " ");
    let collapsed = cleaned.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.is_empty() {
        "unknown".to_string()
    } else {
        collapsed
    }
}

pub async fn subscribe_receivers<V: BenchValue>(
    sender: &KeySender<u64, V>,
    mode: Mode,
    dist: Distribution,
) -> Vec<Vec<KeyReceiver<u64, V>>> {
    let mut receivers = Vec::with_capacity(dist.keys);
    for key in 0..dist.keys {
        let mut per_key = Vec::with_capacity(mode.receivers_per_key);
        for _ in 0..mode.receivers_per_key {
            per_key.push(sender.subscribe(key as u64).await);
        }
        receivers.push(per_key);
    }
    receivers
}

pub async fn send_messages_with_variant<V: BenchValue>(
    sender: &KeySender<u64, V>,
    mode: Mode,
    dist: Distribution,
    variant: SendWithReceiversVariant,
) {
    let total = dist.total_sends();
    let chunk = total.div_ceil(mode.sender_tasks);
    let workers = (0..mode.sender_tasks).map(|task_idx| {
        let start = task_idx * chunk;
        let end = ((task_idx + 1) * chunk).min(total);
        async move {
            for op in start..end {
                let key = (op % dist.keys) as u64;
                let value = V::from_index(op as u64);
                let delivered = match variant {
                    SendWithReceiversVariant::GuardHeld => {
                        sender.__bench_send_guard_held(&key, value).await
                    }
                    SendWithReceiversVariant::RcSender => {
                        sender.__bench_send_rc_sender_lookup(&key, value).await
                    }
                    SendWithReceiversVariant::ArcSender => {
                        sender.__bench_send_arc_sender(&key, value).await
                    }
                };
                std::hint::black_box(delivered);
            }
        }
    });
    join_all(workers).await;
}

pub async fn send_messages_no_receivers<V: BenchValue>(
    sender: &KeySender<u64, V>,
    mode: Mode,
    dist: Distribution,
) {
    let total = dist.total_sends();
    let chunk = total.div_ceil(mode.sender_tasks);
    let workers = (0..mode.sender_tasks).map(|task_idx| {
        let start = task_idx * chunk;
        let end = ((task_idx + 1) * chunk).min(total);
        async move {
            for op in start..end {
                let key = 1_000_000 + dist.keys as u64 + (op % dist.keys) as u64;
                let delivered = sender.send(&key, V::from_index(op as u64)).await;
                std::hint::black_box(delivered);
            }
        }
    });
    join_all(workers).await;
}

pub async fn recv_all<V: BenchValue>(
    receivers: &mut [Vec<KeyReceiver<u64, V>>],
    dist: Distribution,
) {
    for per_key in receivers.iter_mut() {
        for receiver in per_key.iter_mut() {
            for _ in 0..dist.messages_per_key {
                let value = receiver.recv().await.expect("receiver closed unexpectedly");
                std::hint::black_box(value);
            }
        }
    }
}

pub async fn wait_for_cleanup<V: BenchValue>(stream: &KeyStream<u64, V>) -> bool {
    let start = Instant::now();
    for _ in 0..MAX_CLEANUP_YIELDS {
        if stream.n_keys().await == 0 {
            return true;
        }
        if start.elapsed() > MAX_CLEANUP_WAIT {
            return false;
        }
        tokio::task::yield_now().await;
    }
    false
}

pub fn per_op_ns(duration: Duration, count: usize) -> f64 {
    let denom = count.max(1) as f64;
    (duration.as_secs_f64() * 1_000_000_000.0) / denom
}

pub fn min_med_p95(samples: &mut [f64]) -> (f64, f64, f64) {
    samples.sort_by(|a, b| a.partial_cmp(b).expect("nan in sample"));
    let min = samples[0];
    let mid = samples.len() / 2;
    let med = if samples.len().is_multiple_of(2) {
        (samples[mid - 1] + samples[mid]) / 2.0
    } else {
        samples[mid]
    };
    let p95_idx = (samples.len() - 1) * 95 / 100;
    let p95 = samples[p95_idx];
    (min, med, p95)
}

pub fn reset_dropvalue_drops() {
    DROPVALUE_DROPS.with(|count| count.set(0));
}

pub fn dropvalue_drops() -> u64 {
    DROPVALUE_DROPS.with(Cell::get)
}

pub fn csv_prefix(meta: &BenchMetadata) -> String {
    format!(
        "{},{},{},{},{},{},{},{},{},{},{}",
        meta.branch,
        meta.commit,
        meta.arch,
        meta.cpu,
        meta.backend,
        meta.runtime_mode,
        meta.workers,
        meta.criterion_warmup_ms,
        meta.criterion_measurement_ms,
        meta.criterion_sample_size,
        meta.criterion_sampling_mode
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn sanitizing_cpu_field_removes_commas_and_newlines() {
        let sanitized = super::sanitize_csv_field("Intel, Xeon\nPlatinum\t8370C");
        assert!(!sanitized.contains(','));
        assert!(!sanitized.contains('\n'));
    }
}
