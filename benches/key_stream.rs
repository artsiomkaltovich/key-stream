#![allow(clippy::unwrap_used, clippy::expect_used)]

// `local` rows are concurrency without parallelism: `16x1` measures future
// interleaving on one thread, not lock contention, so "`local` wins `16x1`"
// reads backwards. It wins by never paying for contention it also cannot
// exploit. Only `shared` rows measure real contention.
use criterion::{
    BenchmarkId, Criterion, SamplingMode, Throughput, criterion_group, criterion_main,
};
mod common;

use common::{
    BenchMetadata, BenchRuntime, BenchValue, CREATE_BATCH, CRITERION_MEASUREMENT_MS,
    CRITERION_SAMPLE_SIZE, CRITERION_WARMUP_MS, DISTRIBUTIONS, DropValue, MODES, bench_metadata,
    csv_prefix, dropvalue_drops, full_cycle_elements, min_med_p95, per_op_ns,
    reset_dropvalue_drops,
};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::future::Future;
use std::rc::Rc;
use std::time::{Duration, Instant};
use tokio::runtime::Builder;

const PHASES: [&str; 6] = [
    "create",
    "subscribe",
    "send_with_receivers",
    "send_no_receivers",
    "recv",
    "drop",
];

struct PhaseSamples {
    rows: BTreeMap<String, Vec<f64>>,
    attempts: usize,
}

impl PhaseSamples {
    fn new() -> Self {
        Self {
            rows: BTreeMap::new(),
            attempts: 0,
        }
    }

    fn push(&mut self, phase: &str, value: f64) {
        self.rows.entry(phase.to_string()).or_default().push(value);
    }

    fn successful_samples(&self) -> usize {
        self.rows
            .get(PHASES[0])
            .map_or(0, std::vec::Vec::len)
            .min(self.attempts)
    }
}

macro_rules! define_backend_bench {
    (
        $module:ident,
        $backend:literal,
        $backend_mod:ident,
        $runtime_mode:literal,
        $workers:expr,
        $runtime_body:block,
        $drive:item
    ) => {
        mod $module {
            use super::*;
            use key_stream::$backend_mod::{
                KeyReceiver as BackendReceiver, KeySender as BackendSender, KeyStream as BackendStream,
            };

            const BACKEND: &str = $backend;
            const WORKERS: usize = $workers;

            type Stream<V> = BackendStream<u64, V>;
            type Sender<V> = BackendSender<u64, V>;
            type Receiver<V> = BackendReceiver<u64, V>;

            fn runtime() -> BenchRuntime {
                let rt = $runtime_body;
                BenchRuntime {
                    rt,
                    mode: $runtime_mode,
                    workers: WORKERS,
                }
            }

            $drive

            async fn subscribe_receivers<V: BenchValue>(
                sender: &Sender<V>,
                mode: common::Mode,
                dist: common::Distribution,
            ) -> Vec<Vec<Receiver<V>>> {
                let mut receivers = Vec::with_capacity(dist.keys);
                for key in 0..dist.keys {
                    let mut per_key = Vec::with_capacity(mode.receivers_per_key);
                    for _ in 0..mode.receivers_per_key {
                        per_key.push(sender.subscribe(key as u64));
                    }
                    receivers.push(per_key);
                }
                receivers
            }

            async fn send_messages_with_receivers<V: BenchValue>(
                sender: &Sender<V>,
                mode: common::Mode,
                dist: common::Distribution,
            ) {
                let total = dist.total_sends();
                let chunk = total.div_ceil(mode.sender_tasks);
                let tasks: Vec<_> = (0..mode.sender_tasks)
                    .map(|task_idx| {
                        let sender = sender.clone();
                        let start = task_idx * chunk;
                        let end = ((task_idx + 1) * chunk).min(total);
                        async move {
                            for op in start..end {
                                let key = (op % dist.keys) as u64;
                                let delivered = sender.send(&key, V::from_index(op as u64));
                                std::hint::black_box(delivered);
                            }
                        }
                    })
                    .collect();
                drive(tasks).await;
            }

            async fn send_messages_no_receivers<V: BenchValue>(
                sender: &Sender<V>,
                mode: common::Mode,
                dist: common::Distribution,
            ) {
                let total = dist.total_sends();
                let chunk = total.div_ceil(mode.sender_tasks);
                let tasks: Vec<_> = (0..mode.sender_tasks)
                    .map(|task_idx| {
                        let sender = sender.clone();
                        let start = task_idx * chunk;
                        let end = ((task_idx + 1) * chunk).min(total);
                        async move {
                            for op in start..end {
                                let key = 1_000_000 + dist.keys as u64 + (op % dist.keys) as u64;
                                let delivered = sender.send(&key, V::from_index(op as u64));
                                std::hint::black_box(delivered);
                            }
                        }
                    })
                    .collect();
                drive(tasks).await;
            }

            async fn recv_all<V: BenchValue>(
                receivers: &mut [Vec<Receiver<V>>],
                dist: common::Distribution,
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

            async fn run_full_cycle_once<V: BenchValue>(
                mode: common::Mode,
                dist: common::Distribution,
            ) -> (Duration, [f64; PHASES.len()]) {
                let start = Instant::now();

                let create_start = Instant::now();
                let mut stream = None;
                for idx in 0..CREATE_BATCH {
                    let current = Stream::<V>::new(dist.messages_per_key.saturating_add(1).max(2));
                    if idx + 1 == CREATE_BATCH {
                        stream = Some(current);
                    } else {
                        drop(current);
                    }
                }
                let stream = stream.expect("create stream missing");
                let sender = stream.sender();
                let mut phases = [0.0; PHASES.len()];
                phases[0] = per_op_ns(create_start.elapsed(), CREATE_BATCH);

                let subscribe_start = Instant::now();
                let mut receivers = subscribe_receivers(&sender, mode, dist).await;
                phases[1] = per_op_ns(subscribe_start.elapsed(), dist.receivers(mode));

                let send_with_start = Instant::now();
                send_messages_with_receivers(&sender, mode, dist).await;
                phases[2] = per_op_ns(send_with_start.elapsed(), dist.total_sends());

                let send_none_start = Instant::now();
                send_messages_no_receivers(&sender, mode, dist).await;
                phases[3] = per_op_ns(send_none_start.elapsed(), dist.total_sends());

                let recv_start = Instant::now();
                recv_all(&mut receivers, dist).await;
                phases[4] = per_op_ns(recv_start.elapsed(), dist.recv_messages(mode));

                let drop_start = Instant::now();
                drop(receivers);
                assert_eq!(
                    stream.n_keys(),
                    0,
                    "receiver drop cleanup should remove keys synchronously"
                );
                phases[5] = per_op_ns(drop_start.elapsed(), dist.receivers(mode));

                drop(sender);
                drop(stream);
                (start.elapsed(), phases)
            }

            fn bench_full_cycle_for_value<V: BenchValue>(
                group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
                rt: &tokio::runtime::Runtime,
                mode: common::Mode,
                dist: common::Distribution,
                samples: Rc<RefCell<BTreeMap<String, PhaseSamples>>>,
            ) {
                let cell = format!("{}/{}/{}/{}", BACKEND, mode.name, dist.name, V::label());
                let bench_id = BenchmarkId::from_parameter(cell.clone());
                group.throughput(Throughput::Elements(full_cycle_elements(mode, dist)));
                group.bench_function(bench_id, |b| {
                    let samples = Rc::clone(&samples);
                    let cell = cell.clone();
                    b.iter_custom(|iters| {
                        let samples = Rc::clone(&samples);
                        let cell = cell.clone();
                        rt.block_on(async move {
                            let mut total = Duration::ZERO;
                            for _ in 0..iters {
                                let (elapsed, phases) = run_full_cycle_once::<V>(mode, dist).await;
                                total += elapsed;

                                let mut all = samples.borrow_mut();
                                let entry = all.entry(cell.clone()).or_insert_with(PhaseSamples::new);
                                entry.attempts += 1;
                                for (idx, phase) in PHASES.iter().enumerate() {
                                    entry.push(phase, phases[idx]);
                                }
                            }
                            total
                        })
                    });
                });
            }

            pub fn run(
                group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
                samples: Rc<RefCell<BTreeMap<String, PhaseSamples>>>,
            ) -> BenchMetadata {
                let runtime = runtime();
                let meta = bench_metadata(runtime.mode, runtime.workers);
                let rt = runtime.rt;

                for mode in MODES {
                    for dist in DISTRIBUTIONS {
                        bench_full_cycle_for_value::<u64>(
                            group,
                            &rt,
                            mode,
                            dist,
                            Rc::clone(&samples),
                        );
                        bench_full_cycle_for_value::<String>(
                            group,
                            &rt,
                            mode,
                            dist,
                            Rc::clone(&samples),
                        );
                        bench_full_cycle_for_value::<DropValue>(
                            group,
                            &rt,
                            mode,
                            dist,
                            Rc::clone(&samples),
                        );
                    }
                }
                meta
            }
        }
    };
}

define_backend_bench!(
    local_backend,
    "local",
    local,
    "current_thread",
    1,
    {
        Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to build tokio runtime")
    },
    async fn drive<F: Future<Output = ()>>(tasks: Vec<F>) {
        futures::future::join_all(tasks).await;
    }
);

define_backend_bench!(
    shared_backend,
    "shared",
    shared,
    "multi_thread",
    16,
    {
        // Fixed at 16 workers for cross-machine comparability.
        // On small CI runners this intentionally oversubscribes cores.
        Builder::new_multi_thread()
            .worker_threads(WORKERS)
            .enable_all()
            .build()
            .expect("failed to build tokio runtime")
    },
    async fn drive<F: Future<Output = ()> + Send + 'static>(tasks: Vec<F>) {
        let handles: Vec<_> = tasks.into_iter().map(tokio::spawn).collect();
        for handle in handles {
            handle.await.expect("sender task panicked");
        }
    }
);

fn print_phase_breakdown(meta: &BenchMetadata, samples: &mut BTreeMap<String, PhaseSamples>) {
    for (cell, phase_samples) in samples.iter_mut() {
        let successful = phase_samples.successful_samples();
        for phase in PHASES {
            let values = phase_samples
                .rows
                .get_mut(phase)
                .expect("missing phase samples for cell");
            let (min, med, p95) = min_med_p95(values);
            println!(
                "{},phase,{},{},{:.3},{:.3},{:.3},{}",
                csv_prefix(meta),
                phase,
                cell,
                min,
                med,
                p95,
                successful
            );
        }
    }
}

fn bench_full_cycle(c: &mut Criterion) {
    assert!(std::mem::needs_drop::<DropValue>());
    reset_dropvalue_drops();

    let mut group = c.benchmark_group("full_cycle");
    group.sample_size(CRITERION_SAMPLE_SIZE);
    group.warm_up_time(Duration::from_millis(CRITERION_WARMUP_MS));
    group.measurement_time(Duration::from_millis(CRITERION_MEASUREMENT_MS));
    group.sampling_mode(SamplingMode::Flat);

    let local_samples: Rc<RefCell<BTreeMap<String, PhaseSamples>>> =
        Rc::new(RefCell::new(BTreeMap::new()));
    let local_meta = local_backend::run(&mut group, Rc::clone(&local_samples));

    let shared_samples: Rc<RefCell<BTreeMap<String, PhaseSamples>>> =
        Rc::new(RefCell::new(BTreeMap::new()));
    let shared_meta = shared_backend::run(&mut group, Rc::clone(&shared_samples));

    group.finish();

    let ran_dropvalue_cell = local_samples
        .borrow()
        .keys()
        .chain(shared_samples.borrow().keys())
        .any(|cell| cell.ends_with("/DropValue"));
    print_phase_breakdown(&local_meta, &mut local_samples.borrow_mut());
    print_phase_breakdown(&shared_meta, &mut shared_samples.borrow_mut());
    if ran_dropvalue_cell {
        assert!(
            dropvalue_drops() > 0,
            "DropValue drop path was not exercised"
        );
    }
}

criterion_group!(benches, bench_full_cycle);
criterion_main!(benches);
