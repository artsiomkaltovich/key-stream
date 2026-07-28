use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use key_stream::KeyStream;
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
use tokio::task::JoinSet;

fn runtime() -> Runtime {
    Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime")
}

fn bench_subscribe_new_keys(c: &mut Criterion) {
    let rt = runtime();
    let mut group = c.benchmark_group("subscribe_new_keys");
    group.sample_size(60);
    group.measurement_time(Duration::from_secs(8));

    for size in [100usize, 1_000, 10_000] {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_function(BenchmarkId::new("batch", size), |b| {
            b.to_async(&rt).iter(|| async move {
                let stream = KeyStream::<u64, u64>::new(16);
                let sender = stream.sender();
                for key in 0..size as u64 {
                    let receiver = sender.subscribe(key).await;
                    std::hint::black_box(receiver);
                }
            });
        });
    }

    group.finish();
}

fn bench_send_single_key(c: &mut Criterion) {
    let rt = runtime();
    let mut group = c.benchmark_group("send_single_key");
    group.sample_size(60);

    for fanout in [1usize, 10, 100, 1_000] {
        group.throughput(Throughput::Elements(fanout as u64));
        group.bench_function(BenchmarkId::new("fanout", fanout), |b| {
            let (_stream, sender) = rt.block_on(async {
                let stream = KeyStream::<u64, u64>::new(1_024);
                let sender = stream.sender();
                for _ in 0..fanout {
                    let receiver = sender.subscribe(1).await;
                    std::hint::black_box(receiver);
                }
                (stream, sender)
            });

            b.to_async(&rt).iter(|| async {
                let delivered = sender.send(&1, 42).await;
                std::hint::black_box(delivered);
            });
        });
    }

    group.finish();
}

fn bench_subscribe_existing_key(c: &mut Criterion) {
    let rt = runtime();
    let mut group = c.benchmark_group("subscribe_existing_key");
    group.sample_size(60);

    for size in [100usize, 1_000, 10_000] {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_function(BenchmarkId::new("batch", size), |b| {
            let (_stream, sender) = rt.block_on(async {
                let stream = KeyStream::<u64, u64>::new(16);
                let sender = stream.sender();
                // Prime with one subscription so subsequent subscriptions hit the existing-key path.
                let receiver = sender.subscribe(7).await;
                std::hint::black_box(receiver);
                (stream, sender)
            });

            b.to_async(&rt).iter(|| async {
                for _ in 0..size {
                    let receiver = sender.subscribe(7).await;
                    std::hint::black_box(receiver);
                }
            });
        });
    }

    group.finish();
}

fn bench_recv_single_key_many_messages(c: &mut Criterion) {
    let rt = runtime();
    let mut group = c.benchmark_group("recv_single_key_many_messages");
    group.sample_size(60);

    for messages in [100usize, 1_000, 10_000] {
        group.throughput(Throughput::Elements(messages as u64));
        group.bench_function(BenchmarkId::new("messages", messages), |b| {
            b.to_async(&rt).iter(|| async move {
                let stream = KeyStream::<u64, u64>::new(messages + 1);
                let sender = stream.sender();
                let mut receiver = sender.subscribe(1).await;

                for value in 0..messages as u64 {
                    let delivered = sender.send(&1, value).await;
                    std::hint::black_box(delivered);
                }

                for _ in 0..messages {
                    let value = receiver.recv().await.expect("receiver closed unexpectedly");
                    std::hint::black_box(value);
                }
            });
        });
    }

    group.finish();
}

fn bench_recv_many_keys_many_messages(c: &mut Criterion) {
    let rt = runtime();
    let mut group = c.benchmark_group("recv_many_keys_many_messages");
    group.sample_size(60);

    for keys in [4usize, 64, 256] {
        let messages_per_key = 128usize;
        let total_messages = (keys * messages_per_key) as u64;
        group.throughput(Throughput::Elements(total_messages));
        group.bench_function(BenchmarkId::new("keys", keys), |b| {
            b.to_async(&rt).iter(|| async move {
                let stream = KeyStream::<u64, u64>::new(messages_per_key + 1);
                let sender = stream.sender();
                let mut receivers = Vec::with_capacity(keys);

                for key in 0..keys as u64 {
                    receivers.push(sender.subscribe(key).await);
                }

                for message in 0..messages_per_key as u64 {
                    let mut send_set = JoinSet::new();
                    for key in 0..keys as u64 {
                        let sender = sender.clone();
                        send_set.spawn(async move { sender.send(&key, message).await });
                    }
                    while let Some(result) = send_set.join_next().await {
                        std::hint::black_box(result.expect("send task panicked"));
                    }
                }

                for _ in 0..messages_per_key {
                    let mut recv_set = JoinSet::new();
                    for receiver in receivers.drain(..) {
                        recv_set.spawn(async move {
                            let mut receiver = receiver;
                            let value =
                                receiver.recv().await.expect("receiver closed unexpectedly");
                            (receiver, value)
                        });
                    }
                    while let Some(result) = recv_set.join_next().await {
                        let (receiver, value) = result.expect("recv task panicked");
                        std::hint::black_box(value);
                        receivers.push(receiver);
                    }
                }
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_subscribe_new_keys,
    bench_subscribe_existing_key,
    bench_send_single_key,
    bench_recv_single_key_many_messages,
    bench_recv_many_keys_many_messages
);
criterion_main!(benches);
