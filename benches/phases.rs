use futures::future::join_all;
use key_stream::{KeyReceiver, KeySender, KeyStream, Value};
use std::future::Future;
use std::time::{Duration, Instant};
use tokio::runtime::{Builder, Runtime};
use tokio::task::LocalSet;

#[derive(Clone, Copy)]
struct Mode {
    name: &'static str,
    sender_tasks: usize,
    receivers_per_key: usize,
}

#[derive(Clone, Copy)]
struct Distribution {
    name: &'static str,
    keys: usize,
    messages_per_key: usize,
}

const MODES: [Mode; 4] = [
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

const DISTRIBUTIONS: [Distribution; 3] = [
    Distribution {
        name: "1key_10000msg",
        keys: 1,
        messages_per_key: 10_000,
    },
    Distribution {
        name: "100key_100msg",
        keys: 100,
        messages_per_key: 100,
    },
    Distribution {
        name: "10000key_1msg",
        keys: 10_000,
        messages_per_key: 1,
    },
];

const WARMUP_PASSES: usize = 3;
const SAMPLE_PASSES: usize = 50;
const MAX_CLEANUP_YIELDS: usize = 10_000;
const MAX_CLEANUP_WAIT: Duration = Duration::from_secs(1);

trait BenchValue: Value + Clone + 'static {
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

impl Distribution {
    fn total_sends(self) -> usize {
        self.keys * self.messages_per_key
    }

    fn recv_messages(self, mode: Mode) -> usize {
        self.total_sends() * mode.receivers_per_key
    }

    fn receivers(self, mode: Mode) -> usize {
        self.keys * mode.receivers_per_key
    }
}

fn runtime() -> Runtime {
    Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime")
}

async fn run_on_localset<F, T>(fut: F) -> T
where
    F: Future<Output = T>,
{
    LocalSet::new().run_until(fut).await
}

async fn subscribe_receivers<V: BenchValue>(
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

async fn send_messages<V: BenchValue>(
    sender: &KeySender<u64, V>,
    mode: Mode,
    dist: Distribution,
    missing_keys: bool,
) {
    let total = dist.total_sends();
    let chunk = total.div_ceil(mode.sender_tasks);
    let workers = (0..mode.sender_tasks).map(|task_idx| {
        let start = task_idx * chunk;
        let end = ((task_idx + 1) * chunk).min(total);
        async move {
            for op in start..end {
                let base_key = (op % dist.keys) as u64;
                let key = if missing_keys {
                    1_000_000 + dist.keys as u64 + base_key
                } else {
                    base_key
                };
                let delivered = sender.send(&key, V::from_index(op as u64)).await;
                std::hint::black_box(delivered);
            }
        }
    });
    join_all(workers).await;
}

async fn recv_all<V: BenchValue>(receivers: &mut [Vec<KeyReceiver<u64, V>>], dist: Distribution) {
    for per_key in receivers.iter_mut() {
        for receiver in per_key.iter_mut() {
            for _ in 0..dist.messages_per_key {
                let value = receiver.recv().await.expect("receiver closed unexpectedly");
                std::hint::black_box(value);
            }
        }
    }
}

async fn wait_for_cleanup<V: BenchValue>(stream: &KeyStream<u64, V>) {
    let start = Instant::now();
    for _ in 0..MAX_CLEANUP_YIELDS {
        if stream.n_keys().await == 0 {
            return;
        }
        if start.elapsed() > MAX_CLEANUP_WAIT {
            panic!("cleanup timeout waiting for dropped keys");
        }
        tokio::task::yield_now().await;
    }
    panic!("cleanup did not finish within yield budget");
}

fn per_op_ns(duration: Duration, count: usize) -> f64 {
    let denom = count.max(1) as f64;
    (duration.as_secs_f64() * 1_000_000_000.0) / denom
}

async fn run_pass<V: BenchValue>(mode: Mode, dist: Distribution) -> [f64; 6] {
    let mut out = [0.0; 6];

    let create_start = Instant::now();
    let stream = KeyStream::<u64, V>::new(dist.messages_per_key.saturating_add(1).max(2));
    let sender = stream.sender();
    out[0] = per_op_ns(create_start.elapsed(), 1);

    let subscribe_start = Instant::now();
    let mut receivers = subscribe_receivers(&sender, mode, dist).await;
    out[1] = per_op_ns(subscribe_start.elapsed(), dist.receivers(mode));

    let send_with_start = Instant::now();
    send_messages(&sender, mode, dist, false).await;
    out[2] = per_op_ns(send_with_start.elapsed(), dist.total_sends());

    let send_none_start = Instant::now();
    send_messages(&sender, mode, dist, true).await;
    out[3] = per_op_ns(send_none_start.elapsed(), dist.total_sends());

    let recv_start = Instant::now();
    recv_all(&mut receivers, dist).await;
    out[4] = per_op_ns(recv_start.elapsed(), dist.recv_messages(mode));

    let drop_start = Instant::now();
    drop(receivers);
    wait_for_cleanup(&stream).await;
    out[5] = per_op_ns(drop_start.elapsed(), dist.receivers(mode));

    drop(sender);
    drop(stream);

    out
}

fn min_med_max(samples: &mut [f64]) -> (f64, f64, f64) {
    samples.sort_by(|a, b| a.partial_cmp(b).expect("nan in sample"));
    let min = samples[0];
    let max = samples[samples.len() - 1];
    let mid = samples.len() / 2;
    let med = if samples.len().is_multiple_of(2) {
        (samples[mid - 1] + samples[mid]) / 2.0
    } else {
        samples[mid]
    };
    (min, med, max)
}

async fn run_cell<V: BenchValue>(mode: Mode, dist: Distribution) {
    for _ in 0..WARMUP_PASSES {
        let _ = run_pass::<V>(mode, dist).await;
    }

    let mut phases: [Vec<f64>; 6] = std::array::from_fn(|_| Vec::with_capacity(SAMPLE_PASSES));
    for _ in 0..SAMPLE_PASSES {
        let pass = run_pass::<V>(mode, dist).await;
        for (idx, value) in pass.into_iter().enumerate() {
            phases[idx].push(value);
        }
    }

    let phase_names = [
        "create",
        "subscribe",
        "send_with_receivers/guard_held",
        "send_no_receivers",
        "recv",
        "drop",
    ];

    for (idx, name) in phase_names.into_iter().enumerate() {
        let (min, med, max) = min_med_max(&mut phases[idx]);
        println!(
            "{}/{}/{}/{},{:.3},{:.3},{:.3}",
            name,
            mode.name,
            dist.name,
            V::label(),
            min,
            med,
            max
        );
    }
}

fn main() {
    let rt = runtime();
    rt.block_on(async {
        run_on_localset(async {
            for mode in MODES {
                for dist in DISTRIBUTIONS {
                    run_cell::<u64>(mode, dist).await;
                    run_cell::<String>(mode, dist).await;
                }
            }
        })
        .await;
    });
}
