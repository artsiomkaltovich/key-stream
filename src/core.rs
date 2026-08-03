use futures::Stream;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;

pub trait Backend<K, V>: Clone + 'static {
    /// Cheaply clonable handle to one channel, stored as the map value.
    /// Plain refcount, no interior mutability.
    type Handle: Clone + Deref<Target = broadcast::Sender<V>>;
    /// Non-owning handle to the map, held by `KeyReceiverCore` so receivers do not
    /// keep the map alive after the stream and all senders have dropped.
    type WeakSelf: Clone;
    type ReadGuard<'a>: Deref<Target = HashMap<K, Self::Handle>>
    where
        Self: 'a;
    type WriteGuard<'a>: DerefMut<Target = HashMap<K, Self::Handle>>
    where
        Self: 'a;

    fn new() -> Self;
    fn wrap(sender: broadcast::Sender<V>) -> Self::Handle;
    fn read(&self) -> Self::ReadGuard<'_>;
    fn write(&self) -> Self::WriteGuard<'_>;
    fn downgrade(&self) -> Self::WeakSelf;
    fn upgrade(weak: &Self::WeakSelf) -> Option<Self>;
}

pub struct KeyStreamCore<B: Backend<K, V>, K: Eq + Hash, V: Clone> {
    pub(crate) broadcast_capacity: usize,
    pub(crate) streams: B,
    marker: PhantomData<fn() -> (K, V)>,
}

pub struct KeySenderCore<B: Backend<K, V>, K: Eq + Hash, V: Clone> {
    pub(crate) streams: B,
    pub(crate) broadcast_capacity: usize,
    marker: PhantomData<fn() -> (K, V)>,
}

pub struct KeyReceiverCore<B: Backend<K, V>, K: Eq + Hash, V: Clone> {
    pub(crate) key: K,
    pub(crate) streams: B::WeakSelf,
    // Option is only for Drop::drop to take/drop the receiver before acquiring the map guard.
    // Outside Drop, None is unreachable, so accessors report Closed for that state.
    pub(crate) receiver: Option<broadcast::Receiver<V>>,
}

impl<B, K, V> KeyStreamCore<B, K, V>
where
    B: Backend<K, V>,
    K: Eq + Hash,
    V: Clone,
{
    /// Create a new stream with per-key broadcast capacity.
    ///
    /// # Panics
    ///
    /// Panics if `broadcast_capacity == 0` because
    /// [`tokio::sync::broadcast::channel`] requires a positive capacity.
    pub fn new(broadcast_capacity: usize) -> Self {
        Self {
            broadcast_capacity,
            streams: B::new(),
            marker: PhantomData,
        }
    }

    pub fn sender(&self) -> KeySenderCore<B, K, V> {
        KeySenderCore {
            streams: self.streams.clone(),
            broadcast_capacity: self.broadcast_capacity,
            marker: PhantomData,
        }
    }

    pub fn n_keys(&self) -> usize {
        self.streams.read().len()
    }

    pub fn key_capacity(&self) -> usize {
        self.streams.read().capacity()
    }
}

impl<B, K, V> KeySenderCore<B, K, V>
where
    B: Backend<K, V>,
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Sends to all current receivers for the key.
    ///
    /// Returns `0` if the key is absent or if no receivers remain.
    pub fn send(&self, key: &K, value: V) -> usize {
        let sender = {
            let streams = self.streams.read();
            streams.get(key).cloned()
        };
        let Some(sender) = sender else {
            return 0;
        };
        sender.send(value).unwrap_or(0)
    }

    pub fn subscribe(&self, key: K) -> KeyReceiverCore<B, K, V> {
        // subscribe() must stay inside the guarded blocks to serialize against
        // KeyReceiverCore::drop's write guard; hoisting it out opens a race where drop
        // can remove the key before subscribe, leaving a detached receiver that reads Closed.
        let receiver = {
            let streams = self.streams.read();
            streams.get(&key).map(|sender| sender.subscribe())
        };
        let receiver = match receiver {
            Some(receiver) => receiver,
            None => {
                let mut streams = self.streams.write();
                let sender = streams.entry(key.clone()).or_insert_with(|| {
                    let (sender, _) = broadcast::channel(self.broadcast_capacity);
                    B::wrap(sender)
                });
                sender.subscribe()
            }
        };
        KeyReceiverCore {
            key,
            streams: self.streams.downgrade(),
            receiver: Some(receiver),
        }
    }

    pub fn n_keys(&self) -> usize {
        self.streams.read().len()
    }

    pub fn key_capacity(&self) -> usize {
        self.streams.read().capacity()
    }
}

impl<B, K, V> Clone for KeySenderCore<B, K, V>
where
    B: Backend<K, V>,
    K: Eq + Hash,
    V: Clone,
{
    fn clone(&self) -> Self {
        Self {
            streams: self.streams.clone(),
            broadcast_capacity: self.broadcast_capacity,
            marker: PhantomData,
        }
    }
}

impl<B, K, V> KeyReceiverCore<B, K, V>
where
    B: Backend<K, V>,
    K: Eq + Hash,
    V: Clone,
{
    pub async fn recv(&mut self) -> Result<V, RecvError> {
        let Some(receiver) = self.receiver.as_mut() else {
            return Err(RecvError::Closed);
        };
        receiver.recv().await
    }

    pub fn try_recv(&mut self) -> Result<V, broadcast::error::TryRecvError> {
        let Some(receiver) = self.receiver.as_mut() else {
            return Err(broadcast::error::TryRecvError::Closed);
        };
        receiver.try_recv()
    }

    /// Receive by blocking the calling thread.
    ///
    /// # Panics
    ///
    /// Panics if called from within an asynchronous execution context. This is
    /// [`tokio::sync::broadcast::Receiver::blocking_recv`]'s contract, inherited
    /// unchanged: call it from a dedicated thread (for example one from
    /// `spawn_blocking`), never from inside a runtime.
    pub fn blocking_recv(&mut self) -> Result<V, RecvError> {
        let Some(receiver) = self.receiver.as_mut() else {
            return Err(RecvError::Closed);
        };
        receiver.blocking_recv()
    }

    pub fn into_stream(self) -> impl Stream<Item = Result<V, RecvError>> {
        futures::stream::unfold(self, |mut receiver| async {
            match receiver.recv().await {
                Err(RecvError::Closed) => None,
                item => Some((item, receiver)),
            }
        })
    }
}

impl<B, K, V> Drop for KeyReceiverCore<B, K, V>
where
    B: Backend<K, V>,
    K: Eq + Hash,
    V: Clone,
{
    fn drop(&mut self) {
        // Take/drop the receiver first. Dropping it can run unread value destructors
        // that may re-enter this map; holding the guard here is not reorderable.
        let Some(receiver) = self.receiver.take() else {
            return;
        };
        drop(receiver);

        let Some(streams) = B::upgrade(&self.streams) else {
            return;
        };
        let mut streams = streams.write();
        if streams
            .get(&self.key)
            .is_some_and(|sender| sender.receiver_count() == 0)
        {
            streams.remove(&self.key);
            optimize_dict_mem(&mut streams);
        }
    }
}

/// Halve the map's capacity once it is less than half full, so a workload that
/// subscribed to many keys and then released them does not hold the peak
/// allocation forever.
///
/// Two guards keep this from being counterproductive:
///
/// - `cap > 64` leaves small maps alone entirely. Shrinking them saves a trivial
///   amount and the reallocation is pure loss.
/// - halving rather than `shrink_to_fit` leaves headroom, so re-subscribing a few
///   keys does not immediately force a grow.
///
/// Called only from `KeyReceiverCore::drop`, i.e. only when a key was just removed.
///
/// **Known limitation:** a workload that repeatedly cycles the key count across a
/// power-of-two boundary can thrash — grow to `2n`, drop below `n`, shrink to `n`,
/// grow again — paying a rehash each way. The threshold is deliberately not
/// configurable; if that pattern ever shows up in practice, the fix is a smarter
/// policy (hysteresis, or shrinking on a schedule rather than on every removal)
/// rather than a knob for callers to guess at.
pub(crate) fn optimize_dict_mem<K, H>(streams: &mut HashMap<K, H>)
where
    K: Eq + Hash,
{
    let cap = streams.capacity() >> 1;
    let len = streams.len();
    if cap > 64 && len < cap {
        streams.shrink_to(cap);
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::{local, shared};
    use futures::StreamExt;
    use futures::future::join_all;
    use static_assertions::{assert_impl_all, assert_not_impl_any};
    use std::cell::{Cell, RefCell};
    use std::pin::Pin;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, PoisonError, RwLock};
    use tokio::time::{Duration, timeout};

    type LocalStreams<K, V> = Rc<RefCell<HashMap<K, Rc<broadcast::Sender<V>>>>>;
    type SharedStreams<K, V> = Arc<RwLock<HashMap<K, Arc<broadcast::Sender<V>>>>>;

    const TEST_TIMEOUT: Duration = Duration::from_secs(5);

    async fn guarded<F: std::future::Future<Output = ()>>(f: F) {
        timeout(TEST_TIMEOUT, f).await.expect("test timed out");
    }

    async fn recv_body<B: Backend<String, String>>() {
        let key_stream = KeyStreamCore::<B, String, String>::new(10);
        let sender = key_stream.sender();
        let mut receiver = sender.subscribe("1".to_string());
        assert_eq!(key_stream.n_keys(), 1);
        sender.send(&"1".to_string(), "value".to_string());
        assert_eq!(receiver.recv().await.unwrap(), "value".to_string());
    }

    async fn no_receiver_body<B: Backend<String, String>>() {
        let key_stream = KeyStreamCore::<B, String, String>::new(10);
        let sender = key_stream.sender();
        let result = sender.send(&"1".to_string(), "value".to_string());
        assert_eq!(result, 0);
        assert_eq!(key_stream.n_keys(), 0);
    }

    async fn send_after_drop_body<B: Backend<String, String>>() {
        let key_stream = KeyStreamCore::<B, String, String>::new(10);
        let sender = key_stream.sender();
        let receiver = sender.subscribe("1".to_string());
        assert_eq!(key_stream.n_keys(), 1);
        drop(receiver);
        let result = sender.send(&"1".to_string(), "value".to_string());
        assert_eq!(result, 0);
        assert_eq!(key_stream.n_keys(), 0);
    }

    async fn send_after_drop_reclaims_key_immediately_body<B: Backend<String, String>>() {
        let key_stream = KeyStreamCore::<B, String, String>::new(10);
        let sender = key_stream.sender();
        let receiver = sender.subscribe("1".to_string());
        drop(receiver);
        let result = sender.send(&"1".to_string(), "value".to_string());
        assert_eq!(result, 0);
        assert_eq!(key_stream.n_keys(), 0);
    }

    async fn lagged_body<B: Backend<String, String>>() {
        let key_stream = KeyStreamCore::<B, String, String>::new(1);
        let sender = key_stream.sender();
        let mut receiver = sender.subscribe("1".to_string());
        assert_eq!(sender.send(&"1".to_string(), "value1".to_string()), 1);
        assert_eq!(sender.send(&"1".to_string(), "value2".to_string()), 1);
        assert_eq!(receiver.recv().await, Err(RecvError::Lagged(1)));
        assert_eq!(receiver.recv().await.unwrap(), "value2".to_string());
    }

    async fn recv_struct_body<B: Backend<String, MyStruct>>() {
        let key_stream = KeyStreamCore::<B, String, MyStruct>::new(10);
        let sender = key_stream.sender();
        let mut receiver = sender.subscribe("1".to_string());
        assert_eq!(key_stream.n_keys(), 1);
        sender.send(
            &"1".to_string(),
            MyStruct {
                field1: "value".to_string(),
                field2: 42,
            },
        );
        let received = receiver.recv().await.unwrap();
        assert_eq!(received.field1, "value".to_string());
        assert_eq!(received.field2, 42);
    }

    async fn messages_broadcasted_body<B: Backend<String, String>>() {
        let key_stream = KeyStreamCore::<B, String, String>::new(10);
        let sender = key_stream.sender();
        let mut receiver1 = sender.subscribe("1".to_string());
        let mut receiver2 = sender.subscribe("1".to_string());
        assert_eq!(key_stream.n_keys(), 1);
        sender.send(&"1".to_string(), "value".to_string());
        assert_eq!(receiver1.recv().await.unwrap(), "value".to_string());
        assert_eq!(receiver2.recv().await.unwrap(), "value".to_string());
    }

    async fn key_filter_body<B: Backend<String, String>>() {
        let key_stream = KeyStreamCore::<B, String, String>::new(10);
        let sender = key_stream.sender();
        let mut receiver1 = sender.subscribe("1".to_string());
        let mut receiver2 = sender.subscribe("2".to_string());
        assert_eq!(key_stream.n_keys(), 2);
        sender.send(&"1".to_string(), "value1".to_string());
        sender.send(&"2".to_string(), "value2".to_string());
        assert_eq!(receiver1.recv().await.unwrap(), "value1".to_string());
        assert_eq!(receiver2.recv().await.unwrap(), "value2".to_string());
        assert_eq!(
            receiver1.try_recv(),
            Err(broadcast::error::TryRecvError::Empty)
        );
        assert_eq!(
            receiver2.try_recv(),
            Err(broadcast::error::TryRecvError::Empty)
        );
    }

    async fn key_drop_body<B: Backend<String, String>>() {
        let key_stream = KeyStreamCore::<B, String, String>::new(10);
        let sender = key_stream.sender();
        let receiver = sender.subscribe("1".to_string());
        assert_eq!(key_stream.n_keys(), 1);
        sender.send(&"1".to_string(), "value".to_string());
        drop(receiver);
        assert_eq!(key_stream.n_keys(), 0);
    }

    async fn key_non_dropped_if_other_receiver_exists_body<B: Backend<String, String>>() {
        let key_stream = KeyStreamCore::<B, String, String>::new(10);
        let sender = key_stream.sender();
        let receiver1 = sender.subscribe("1".to_string());
        let _receiver2 = sender.subscribe("1".to_string());
        assert_eq!(key_stream.n_keys(), 1);
        sender.send(&"1".to_string(), "value".to_string());
        drop(receiver1);
        assert_eq!(key_stream.n_keys(), 1);
    }

    async fn two_clients_body<B: Backend<String, String>>() {
        let key_stream = KeyStreamCore::<B, String, String>::new(10);
        let sender1 = key_stream.sender();
        let sender2 = key_stream.sender();
        let mut receiver1 = sender1.subscribe("1".to_string());
        let mut receiver2 = sender2.subscribe("1".to_string());
        assert_eq!(key_stream.n_keys(), 1);
        sender1.send(&"1".to_string(), "value".to_string());
        assert_eq!(receiver1.recv().await.unwrap(), "value".to_string());
        assert_eq!(receiver2.recv().await.unwrap(), "value".to_string());
        assert_eq!(
            receiver1.try_recv(),
            Err(broadcast::error::TryRecvError::Empty)
        );
        assert_eq!(
            receiver2.try_recv(),
            Err(broadcast::error::TryRecvError::Empty)
        );
        drop(sender2);
        assert_eq!(key_stream.n_keys(), 1);
        sender1.send(&"1".to_string(), "value".to_string());
        assert_eq!(receiver1.recv().await.unwrap(), "value".to_string());
    }

    async fn reconnect_body<B: Backend<String, String>>() {
        let key_stream = KeyStreamCore::<B, String, String>::new(10);
        let sender = key_stream.sender();
        let mut receiver1 = sender.subscribe("1".to_string());
        assert_eq!(key_stream.n_keys(), 1);
        sender.send(&"1".to_string(), "value".to_string());
        assert_eq!(receiver1.recv().await.unwrap(), "value".to_string());
        drop(receiver1);
        assert_eq!(key_stream.n_keys(), 0);
        let mut receiver2 = sender.subscribe("1".to_string());
        assert_eq!(key_stream.n_keys(), 1);
        sender.send(&"1".to_string(), "value2".to_string());
        assert_eq!(receiver2.recv().await.unwrap(), "value2".to_string());
    }

    async fn resubscribe_after_drop_recreates_key_body<B: Backend<String, String>>() {
        let key_stream = KeyStreamCore::<B, String, String>::new(10);
        let sender = key_stream.sender();
        let receiver1 = sender.subscribe("1".to_string());
        assert_eq!(key_stream.n_keys(), 1);
        drop(receiver1);
        assert_eq!(key_stream.n_keys(), 0);
        let mut receiver2 = sender.subscribe("1".to_string());
        assert_eq!(key_stream.n_keys(), 1);
        let result = sender.send(&"1".to_string(), "value".to_string());
        assert_eq!(result, 1);
        assert_eq!(receiver2.recv().await.unwrap(), "value".to_string());
    }

    async fn shrink_dict_body<B: Backend<i32, String>>() {
        let key_stream = KeyStreamCore::<B, i32, String>::new(1);
        let sender = key_stream.sender();
        let tasks = (0..500).map(|i| {
            let sender = sender.clone();
            async move { sender.subscribe(i) }
        });
        let mut subs = join_all(tasks).await;
        assert_eq!(key_stream.n_keys(), 500);
        subs.drain(0..400);
        assert_eq!(key_stream.n_keys(), 100);
        assert!(key_stream.key_capacity() < 500);
    }

    async fn drop_sender_body<B: Backend<String, String>>() {
        let key_stream = KeyStreamCore::<B, String, String>::new(10);
        let sender = key_stream.sender();
        let mut receiver = sender.subscribe("1".to_string());
        assert_eq!(key_stream.n_keys(), 1);
        sender.send(&"1".to_string(), "value".to_string());
        drop(sender);
        assert_eq!(receiver.recv().await.unwrap(), "value".to_string());
        assert_eq!(
            receiver.try_recv(),
            Err(broadcast::error::TryRecvError::Empty)
        );
    }

    async fn drop_stream_body<B: Backend<String, String>>() {
        let key_stream = KeyStreamCore::<B, String, String>::new(10);
        let sender = key_stream.sender();
        drop(key_stream);
        let mut receiver = sender.subscribe("1".to_string());
        sender.send(&"1".to_string(), "value".to_string());
        assert_eq!(receiver.recv().await.unwrap(), "value".to_string());
        assert_eq!(
            receiver.try_recv(),
            Err(broadcast::error::TryRecvError::Empty)
        );
    }

    async fn len_and_capacity_body<B: Backend<String, String>>() {
        let key_stream = KeyStreamCore::<B, String, String>::new(10);
        let sender1 = key_stream.sender();
        let sender2 = key_stream.sender();
        assert_eq!(key_stream.n_keys(), 0);
        assert_eq!(sender1.n_keys(), 0);
        assert_eq!(sender2.n_keys(), 0);
        assert_eq!(key_stream.key_capacity(), sender1.key_capacity());
        assert_eq!(key_stream.key_capacity(), sender2.key_capacity());
        let _receiver1 = sender1.subscribe("1".to_string());
        let _receiver2 = sender2.subscribe("2".to_string());
        let _receiver3 = sender2.subscribe("3".to_string());
        assert_eq!(key_stream.n_keys(), 3);
        assert_eq!(sender1.n_keys(), 3);
        assert_eq!(sender2.n_keys(), 3);
        assert_eq!(key_stream.key_capacity(), sender1.key_capacity());
        assert_eq!(key_stream.key_capacity(), sender2.key_capacity());
    }

    async fn stream_body<B: Backend<String, String>>() {
        type WatchStream =
            Pin<Box<dyn futures::Stream<Item = Result<String, RecvError>> + 'static>>;
        let key_stream = KeyStreamCore::<B, String, String>::new(10);
        let sender = key_stream.sender();
        let receiver = sender.subscribe("1".to_string());
        sender.send(&"1".to_string(), "value1".to_string());
        let stream = receiver.into_stream();
        let stream = Box::pin(stream) as WatchStream;
        sender.send(&"1".to_string(), "value2".to_string());
        let msg = timeout(Duration::from_secs(1), stream.take(2).collect::<Vec<_>>()).await;
        let msg = msg.expect("timeout");
        assert_eq!(
            msg,
            vec![Ok("value1".to_string()), Ok("value2".to_string())]
        );
    }

    async fn stream_ends_when_channel_closes_body<B: Backend<String, String>>() {
        let key_stream = KeyStreamCore::<B, String, String>::new(10);
        let sender = key_stream.sender();
        let receiver = sender.subscribe("1".to_string());
        sender.send(&"1".to_string(), "value".to_string());
        drop(sender);
        drop(key_stream);
        let stream = receiver.into_stream();
        let msg = timeout(Duration::from_secs(1), stream.collect::<Vec<_>>()).await;
        let msg = msg.expect("stream did not terminate after close");
        assert_eq!(msg, vec![Ok("value".to_string())]);
    }

    async fn public_send_reentrant_drop_no_panic_body<B, V, Build, Make, Arm>(build: Build)
    where
        B: Backend<u64, V> + Clone,
        V: Clone + 'static,
        Build: Fn(B) -> (Make, Arm),
        Make: Fn() -> V,
        Arm: Fn(bool),
    {
        let key_stream = KeyStreamCore::<B, u64, V>::new(1);
        let sender = key_stream.sender();
        let (make_value, arm) = build(key_stream.streams.clone());
        let mut lagging_receiver = sender.subscribe(1);
        assert_eq!(sender.send(&1, make_value()), 1);
        std::hint::black_box(&mut lagging_receiver);
        arm(true);
        assert_eq!(sender.send(&1, make_value()), 1);
    }

    async fn receiver_drop_take_first_prevents_reentrant_borrow_panic_body<B, V, Build, Make>(
        build: Build,
    ) where
        B: Backend<u64, V> + Clone,
        V: Clone + 'static,
        Build: Fn(B) -> Make,
        Make: Fn() -> V,
    {
        let key_stream = KeyStreamCore::<B, u64, V>::new(1);
        let sender = key_stream.sender();
        let make_value = build(key_stream.streams.clone());
        let receiver = sender.subscribe(1);
        assert_eq!(sender.send(&1, make_value()), 1);
        drop(receiver);
        assert_eq!(key_stream.n_keys(), 0);
    }

    async fn concurrency_body<B: Backend<String, String>>() {
        let key_stream = KeyStreamCore::<B, String, String>::new(8);
        let sender = key_stream.sender();
        let keys = vec![
            "k0".to_string(),
            "k1".to_string(),
            "k2".to_string(),
            "k0".to_string(),
            "k1".to_string(),
            "k2".to_string(),
            "k0".to_string(),
            "k1".to_string(),
            "k2".to_string(),
            "k0".to_string(),
            "k1".to_string(),
            "k2".to_string(),
        ];

        let tasks = keys.into_iter().enumerate().map(|(i, key)| {
            let sender = sender.clone();
            async move {
                let mut receiver = sender.subscribe(key.clone());
                let value = format!("value-{i}");
                assert!(sender.send(&key, value) > 0);
                receiver.recv().await.expect("recv failed")
            }
        });

        let results = join_all(tasks).await;
        assert_eq!(results.len(), 12);
        assert_eq!(key_stream.n_keys(), 0);
    }

    #[derive(Clone, Debug)]
    struct MyStruct {
        field1: String,
        field2: i32,
    }

    #[derive(Clone)]
    struct LocalReentrantDrop {
        armed: Rc<Cell<bool>>,
        on_drop: Rc<dyn Fn()>,
    }

    impl Drop for LocalReentrantDrop {
        fn drop(&mut self) {
            if self.armed.replace(false) {
                (self.on_drop)();
            }
        }
    }

    #[derive(Clone)]
    struct SharedReentrantDrop {
        armed: Arc<AtomicBool>,
        on_drop: Arc<dyn Fn() + Send + Sync>,
    }

    impl Drop for SharedReentrantDrop {
        fn drop(&mut self) {
            if self.armed.swap(false, Ordering::SeqCst) {
                (self.on_drop)();
            }
        }
    }

    #[tokio::test]
    async fn recv_local() {
        guarded(recv_body::<LocalStreams<String, String>>()).await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn recv_shared() {
        guarded(recv_body::<SharedStreams<String, String>>()).await
    }
    #[tokio::test]
    async fn no_receiver_local() {
        guarded(no_receiver_body::<LocalStreams<String, String>>()).await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn no_receiver_shared() {
        guarded(no_receiver_body::<SharedStreams<String, String>>()).await
    }
    #[tokio::test]
    async fn send_after_drop_local() {
        guarded(send_after_drop_body::<LocalStreams<String, String>>()).await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn send_after_drop_shared() {
        guarded(send_after_drop_body::<SharedStreams<String, String>>()).await
    }
    #[tokio::test]
    async fn send_after_drop_reclaims_key_immediately_local() {
        guarded(send_after_drop_reclaims_key_immediately_body::<
            LocalStreams<String, String>,
        >())
        .await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn send_after_drop_reclaims_key_immediately_shared() {
        guarded(send_after_drop_reclaims_key_immediately_body::<
            SharedStreams<String, String>,
        >())
        .await
    }
    #[tokio::test]
    async fn lagged_local() {
        guarded(lagged_body::<LocalStreams<String, String>>()).await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn lagged_shared() {
        guarded(lagged_body::<SharedStreams<String, String>>()).await
    }
    #[tokio::test]
    async fn recv_struct_local() {
        guarded(recv_struct_body::<LocalStreams<String, MyStruct>>()).await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn recv_struct_shared() {
        guarded(recv_struct_body::<SharedStreams<String, MyStruct>>()).await
    }
    #[tokio::test]
    async fn messages_broadcasted_local() {
        guarded(messages_broadcasted_body::<LocalStreams<String, String>>()).await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn messages_broadcasted_shared() {
        guarded(messages_broadcasted_body::<SharedStreams<String, String>>()).await
    }
    #[tokio::test]
    async fn key_filter_local() {
        guarded(key_filter_body::<LocalStreams<String, String>>()).await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn key_filter_shared() {
        guarded(key_filter_body::<SharedStreams<String, String>>()).await
    }
    #[tokio::test]
    async fn key_drop_local() {
        guarded(key_drop_body::<LocalStreams<String, String>>()).await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn key_drop_shared() {
        guarded(key_drop_body::<SharedStreams<String, String>>()).await
    }
    #[tokio::test]
    async fn key_non_dropped_if_other_receiver_exists_local() {
        guarded(key_non_dropped_if_other_receiver_exists_body::<
            LocalStreams<String, String>,
        >())
        .await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn key_non_dropped_if_other_receiver_exists_shared() {
        guarded(key_non_dropped_if_other_receiver_exists_body::<
            SharedStreams<String, String>,
        >())
        .await
    }
    #[tokio::test]
    async fn two_clients_local() {
        guarded(two_clients_body::<LocalStreams<String, String>>()).await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn two_clients_shared() {
        guarded(two_clients_body::<SharedStreams<String, String>>()).await
    }
    #[tokio::test]
    async fn reconnect_local() {
        guarded(reconnect_body::<LocalStreams<String, String>>()).await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn reconnect_shared() {
        guarded(reconnect_body::<SharedStreams<String, String>>()).await
    }
    #[tokio::test]
    async fn resubscribe_after_drop_recreates_key_local() {
        guarded(resubscribe_after_drop_recreates_key_body::<
            LocalStreams<String, String>,
        >())
        .await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn resubscribe_after_drop_recreates_key_shared() {
        guarded(resubscribe_after_drop_recreates_key_body::<
            SharedStreams<String, String>,
        >())
        .await
    }
    #[tokio::test]
    async fn shrink_dict_local() {
        guarded(shrink_dict_body::<LocalStreams<i32, String>>()).await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn shrink_dict_shared() {
        guarded(shrink_dict_body::<SharedStreams<i32, String>>()).await
    }
    #[tokio::test]
    async fn drop_sender_local() {
        guarded(drop_sender_body::<LocalStreams<String, String>>()).await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn drop_sender_shared() {
        guarded(drop_sender_body::<SharedStreams<String, String>>()).await
    }
    #[tokio::test]
    async fn drop_stream_local() {
        guarded(drop_stream_body::<LocalStreams<String, String>>()).await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn drop_stream_shared() {
        guarded(drop_stream_body::<SharedStreams<String, String>>()).await
    }
    #[tokio::test]
    async fn len_and_capacity_local() {
        guarded(len_and_capacity_body::<LocalStreams<String, String>>()).await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn len_and_capacity_shared() {
        guarded(len_and_capacity_body::<SharedStreams<String, String>>()).await
    }
    #[tokio::test]
    async fn stream_local() {
        guarded(stream_body::<LocalStreams<String, String>>()).await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn stream_shared() {
        guarded(stream_body::<SharedStreams<String, String>>()).await
    }
    #[tokio::test]
    async fn stream_ends_when_channel_closes_local() {
        guarded(stream_ends_when_channel_closes_body::<
            LocalStreams<String, String>,
        >())
        .await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn stream_ends_when_channel_closes_shared() {
        guarded(stream_ends_when_channel_closes_body::<
            SharedStreams<String, String>,
        >())
        .await
    }
    #[tokio::test]
    async fn public_send_reentrant_drop_no_panic_local() {
        guarded(public_send_reentrant_drop_no_panic_body::<
            LocalStreams<u64, LocalReentrantDrop>,
            LocalReentrantDrop,
            _,
            _,
            _,
        >(move |streams| {
            let armed = Rc::new(Cell::new(false));
            let on_drop: Rc<dyn Fn()> = Rc::new(move || {
                let _guard = streams.borrow_mut();
            });
            let make_value = {
                let armed = Rc::clone(&armed);
                let on_drop = Rc::clone(&on_drop);
                move || LocalReentrantDrop {
                    armed: Rc::clone(&armed),
                    on_drop: Rc::clone(&on_drop),
                }
            };
            let arm = {
                let armed = Rc::clone(&armed);
                move |value| armed.set(value)
            };
            (make_value, arm)
        }))
        .await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn public_send_reentrant_drop_no_panic_shared() {
        guarded(public_send_reentrant_drop_no_panic_body::<
            SharedStreams<u64, SharedReentrantDrop>,
            SharedReentrantDrop,
            _,
            _,
            _,
        >(move |streams| {
            let armed = Arc::new(AtomicBool::new(false));
            let on_drop: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
                let _guard = RwLock::write(&streams).unwrap_or_else(PoisonError::into_inner);
            });
            let make_value = {
                let armed = Arc::clone(&armed);
                let on_drop = Arc::clone(&on_drop);
                move || SharedReentrantDrop {
                    armed: Arc::clone(&armed),
                    on_drop: Arc::clone(&on_drop),
                }
            };
            let arm = {
                let armed = Arc::clone(&armed);
                move |value| armed.store(value, Ordering::SeqCst)
            };
            (make_value, arm)
        }))
        .await
    }
    #[tokio::test]
    async fn receiver_drop_take_first_prevents_reentrant_borrow_panic_local() {
        guarded(
            receiver_drop_take_first_prevents_reentrant_borrow_panic_body::<
                LocalStreams<u64, LocalReentrantDrop>,
                LocalReentrantDrop,
                _,
                _,
            >(move |streams| {
                let armed = Rc::new(Cell::new(true));
                let on_drop: Rc<dyn Fn()> = Rc::new(move || {
                    let _guard = streams.borrow_mut();
                });
                {
                    let armed = Rc::clone(&armed);
                    let on_drop = Rc::clone(&on_drop);
                    move || LocalReentrantDrop {
                        armed: Rc::clone(&armed),
                        on_drop: Rc::clone(&on_drop),
                    }
                }
            }),
        )
        .await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn receiver_drop_take_first_prevents_reentrant_borrow_panic_shared() {
        guarded(
            receiver_drop_take_first_prevents_reentrant_borrow_panic_body::<
                SharedStreams<u64, SharedReentrantDrop>,
                SharedReentrantDrop,
                _,
                _,
            >(move |streams| {
                let armed = Arc::new(AtomicBool::new(true));
                let on_drop: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
                    let _guard = RwLock::write(&streams).unwrap_or_else(PoisonError::into_inner);
                });
                {
                    let armed = Arc::clone(&armed);
                    let on_drop = Arc::clone(&on_drop);
                    move || SharedReentrantDrop {
                        armed: Arc::clone(&armed),
                        on_drop: Arc::clone(&on_drop),
                    }
                }
            }),
        )
        .await
    }
    #[tokio::test]
    async fn concurrency_local() {
        guarded(concurrency_body::<LocalStreams<String, String>>()).await
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn concurrency_shared() {
        guarded(concurrency_body::<SharedStreams<String, String>>()).await
    }
    async fn recv_rc_struct_local_body() {
        #[derive(Debug)]
        struct RcStruct {
            field1: String,
            field2: i32,
        }
        let key_stream = local::KeyStream::<String, Rc<RcStruct>>::new(10);
        let sender = key_stream.sender();
        let mut receiver = sender.subscribe("1".to_string());
        assert_eq!(key_stream.n_keys(), 1);
        sender.send(
            &"1".to_string(),
            Rc::new(RcStruct {
                field1: "value".to_string(),
                field2: 42,
            }),
        );
        let received = receiver.recv().await.unwrap();
        assert_eq!(received.field1, "value");
        assert_eq!(received.field2, 42);
    }
    #[tokio::test]
    async fn recv_rc_struct_local() {
        guarded(recv_rc_struct_local_body()).await
    }

    async fn shared_contention_multi_thread_only_body() {
        const N_KEYS: usize = 4;
        const N_TASKS: usize = 32;
        // Capacity must exceed tasks / keys so each per-key receiver can consume
        // one message from every sender without lagging.
        const CAPACITY: usize = 64;

        let key_stream = shared::KeyStream::<String, String>::new(CAPACITY);
        let sender = key_stream.sender();
        let tasks = (0..N_TASKS).map(|i| {
            let sender = sender.clone();
            tokio::spawn(async move {
                let key = format!("k{}", i % N_KEYS);
                let mut receiver = sender.subscribe(key.clone());
                assert!(sender.send(&key, format!("value-{i}")) > 0);
                let _ = receiver.recv().await.expect("recv failed");
            })
        });
        for task in tasks {
            task.await.expect("task failed");
        }
        assert_eq!(key_stream.n_keys(), 0);
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn shared_contention_multi_thread_only() {
        guarded(shared_contention_multi_thread_only_body()).await
    }

    #[test]
    fn static_backend_assertions() {
        assert_impl_all!(shared::KeyStream<u64, u64>: Send, Sync);
        assert_impl_all!(shared::KeySender<u64, u64>: Send, Sync);
        assert_not_impl_any!(local::KeyStream<u64, u64>: Send, Sync);
    }
}
