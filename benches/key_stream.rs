use criterion::{
    BenchmarkId, Criterion, SamplingMode, Throughput, criterion_group, criterion_main,
};
mod common;

use common::{
    BenchMetadata, BenchValue, CREATE_BATCH, CRITERION_MEASUREMENT_MS, CRITERION_SAMPLE_SIZE,
    CRITERION_WARMUP_MS, DISTRIBUTIONS, DropValue, MODES, bench_metadata, csv_prefix,
    dropvalue_drops, full_cycle_elements, min_med_p95, per_op_ns, recv_all, reset_dropvalue_drops,
    run_on_localset, runtime, send_messages_no_receivers, send_messages_with_variant,
    send_variant_label, send_variants, subscribe_receivers, wait_for_cleanup,
};
use key_stream::KeyStream;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

struct PhaseSamples {
    rows: BTreeMap<String, Vec<f64>>,
    attempts: usize,
    discarded_cleanup_timeout: usize,
}

impl PhaseSamples {
    fn new() -> Self {
        Self {
            rows: BTreeMap::new(),
            attempts: 0,
            discarded_cleanup_timeout: 0,
        }
    }

    fn push(&mut self, phase: &str, value: f64) {
        self.rows.entry(phase.to_string()).or_default().push(value);
    }

    fn successful_samples(&self) -> usize {
        self.rows.values().next().map_or(0, Vec::len)
    }
}

enum DiscardReason {
    CleanupTimeout,
}

async fn run_full_cycle_once<V: BenchValue>(
    mode: common::Mode,
    dist: common::Distribution,
    pass_idx: usize,
) -> (Duration, Result<Vec<f64>, DiscardReason>) {
    let start = Instant::now();

    let create_start = Instant::now();
    let mut stream = None;
    for idx in 0..CREATE_BATCH {
        let current = KeyStream::<u64, V>::new(dist.messages_per_key.saturating_add(1).max(2));
        if idx + 1 == CREATE_BATCH {
            stream = Some(current);
        } else {
            drop(current);
        }
    }
    let stream = stream.expect("create stream missing");
    let sender = stream.sender();
    let variants = send_variants();
    let n_variants = variants.len();
    let mut phases = Vec::with_capacity(5 + n_variants);
    phases.push(per_op_ns(create_start.elapsed(), CREATE_BATCH));

    let subscribe_start = Instant::now();
    let mut receivers = subscribe_receivers(&sender, mode, dist).await;
    phases.push(per_op_ns(subscribe_start.elapsed(), dist.receivers(mode)));

    let rotation = pass_idx % n_variants;
    let mut send_durations = vec![Duration::ZERO; n_variants];
    let mut recv_total = Duration::ZERO;
    for offset in 0..n_variants {
        let variant = variants[(rotation + offset) % n_variants];
        let send_start = Instant::now();
        send_messages_with_variant(&sender, mode, dist, variant).await;
        let send_elapsed = send_start.elapsed();
        let idx = variants
            .iter()
            .position(|candidate| *candidate == variant)
            .expect("variant missing from list");
        send_durations[idx] = send_elapsed;

        let recv_start = Instant::now();
        recv_all(&mut receivers, dist).await;
        recv_total += recv_start.elapsed();
    }
    for duration in send_durations {
        phases.push(per_op_ns(duration, dist.total_sends()));
    }

    let send_none_start = Instant::now();
    send_messages_no_receivers(&sender, mode, dist).await;
    phases.push(per_op_ns(send_none_start.elapsed(), dist.total_sends()));

    phases.push(per_op_ns(recv_total, dist.recv_messages(mode) * n_variants));

    let drop_start = Instant::now();
    drop(receivers);
    if !wait_for_cleanup(&stream).await {
        return (start.elapsed(), Err(DiscardReason::CleanupTimeout));
    }
    phases.push(per_op_ns(drop_start.elapsed(), dist.receivers(mode)));

    drop(sender);
    drop(stream);
    (start.elapsed(), Ok(phases))
}

fn bench_full_cycle_for_value<V: BenchValue>(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    rt: &tokio::runtime::Runtime,
    mode: common::Mode,
    dist: common::Distribution,
    samples: Rc<RefCell<BTreeMap<String, PhaseSamples>>>,
) {
    let cell = format!("{}/{}/{}", mode.name, dist.name, V::label());
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
                for pass_idx in 0..iters {
                    let (elapsed, pass) =
                        run_on_localset(run_full_cycle_once::<V>(mode, dist, pass_idx as usize))
                            .await;
                    total += elapsed;

                    let mut all = samples.borrow_mut();
                    let entry = all.entry(cell.clone()).or_insert_with(PhaseSamples::new);
                    entry.attempts += 1;
                    match pass {
                        Ok(phases) => {
                            entry.push("create", phases[0]);
                            entry.push("subscribe", phases[1]);
                            let variants = send_variants();
                            for (idx, variant) in variants.iter().enumerate() {
                                entry.push(
                                    &format!(
                                        "send_with_receivers/{}",
                                        send_variant_label(*variant)
                                    ),
                                    phases[2 + idx],
                                );
                            }
                            let send_none_idx = 2 + variants.len();
                            let recv_idx = send_none_idx + 1;
                            let drop_idx = recv_idx + 1;
                            entry.push("send_no_receivers", phases[send_none_idx]);
                            entry.push("recv", phases[recv_idx]);
                            entry.push("drop", phases[drop_idx]);
                        }
                        Err(DiscardReason::CleanupTimeout) => {
                            entry.discarded_cleanup_timeout += 1;
                        }
                    }
                }
                total
            })
        });
    });
}

fn print_phase_breakdown(meta: &BenchMetadata, samples: &mut BTreeMap<String, PhaseSamples>) {
    for (cell, phase_samples) in samples.iter_mut() {
        let successful = phase_samples.successful_samples();
        if phase_samples.discarded_cleanup_timeout > 0 {
            println!(
                "{},failed,{},cleanup_timeout,{},{},{}",
                csv_prefix(meta),
                cell,
                phase_samples.discarded_cleanup_timeout,
                phase_samples.attempts,
                successful
            );
        }

        for (phase, values) in phase_samples.rows.iter_mut() {
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
    let runtime = runtime();
    let meta = bench_metadata(runtime.mode, runtime.workers);
    let rt = runtime.rt;
    let samples: Rc<RefCell<BTreeMap<String, PhaseSamples>>> =
        Rc::new(RefCell::new(BTreeMap::new()));

    let mut group = c.benchmark_group("full_cycle");
    group.sample_size(CRITERION_SAMPLE_SIZE);
    group.warm_up_time(Duration::from_millis(CRITERION_WARMUP_MS));
    group.measurement_time(Duration::from_millis(CRITERION_MEASUREMENT_MS));
    group.sampling_mode(SamplingMode::Flat);

    for mode in MODES {
        for dist in DISTRIBUTIONS {
            bench_full_cycle_for_value::<u64>(&mut group, &rt, mode, dist, Rc::clone(&samples));
            bench_full_cycle_for_value::<String>(&mut group, &rt, mode, dist, Rc::clone(&samples));
            bench_full_cycle_for_value::<DropValue>(
                &mut group,
                &rt,
                mode,
                dist,
                Rc::clone(&samples),
            );
        }
    }

    group.finish();

    let ran_dropvalue_cell = samples
        .borrow()
        .keys()
        .any(|cell| cell.ends_with("/DropValue"));
    print_phase_breakdown(&meta, &mut samples.borrow_mut());
    if ran_dropvalue_cell {
        assert!(
            dropvalue_drops() > 0,
            "DropValue drop path was not exercised"
        );
    }
}

criterion_group!(benches, bench_full_cycle);
criterion_main!(benches);
