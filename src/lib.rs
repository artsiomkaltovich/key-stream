//! Async key-based message streaming library.
//!
//! Enables sending messages to multiple receivers, grouped by keys, with automatic cleanup of unused keys.
//! No messages are sent until a key is subscribed to, and keys are automatically removed when all receivers are dropped.
//! Keys are reclaimed synchronously when the last receiver for a key is dropped.
//! The memory usage of the keys map is optimized by shrinking it when many keys are removed.
//!
//! The main entry point is [`KeyStream`], created with [`KeyStream::new`].
//! It manages the keys and removes idle keys when receiver drops leave no subscribers.
//! Use [`KeyStream::sender`] to obtain a sender handle for sending messages and subscribing to keys.
//! Use [`KeySender::send`] to send messages to a key, and [`KeySender::subscribe`] to subscribe to a key and receive a [`KeyReceiver`] for that key.
//! [`KeyReceiver::recv`] can be used to receive messages for a key, waiting asynchronously until a message is available.
//! [`KeyReceiver`] can also be converted into a stream of messages using [`KeyReceiver::into_stream`].
//!
//! # Examples
//!
//! ```
//! use key_stream::KeyStream;
//! use tokio;
//!
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() {
//!     let key_stream = KeyStream::<i32, String>::new(10);
//!     let sender = key_stream.sender();
//!     let mut receiver = sender.subscribe(1).await;
//!     sender.send(&1, "value".to_string()).await;
//!     assert_eq!(receiver.recv().await.unwrap(), "value".to_string());
//! # }
//! ```
//!
//! ## Key cleanup example
//!
//! ```rust
//! use key_stream::KeyStream;
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() {
//!     let key_stream = KeyStream::<i32, String>::new(10);
//!     let sender = key_stream.sender();
//!     let receiver = sender.subscribe(1).await;
//!     assert_eq!(key_stream.n_keys().await, 1);
//!     drop(receiver);
//!     let result = sender.send(&1, "value".to_string()).await;
//!     assert_eq!(result, 0);
//!     assert_eq!(key_stream.n_keys().await, 0);
//! # }
//! ```
use futures::Stream;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};
#[cfg(feature = "bench-variants")]
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;

/// Trait bound for types usable as keys in [`KeyStream`].
/// Must be hashable, comparable, cloneable, thread-safe, and `'static`.
pub trait Key: Hash + Eq + Clone + 'static {}

/// Trait bound for types usable as values in [`KeyStream`].
/// Must be cloneable, thread-safe, and `'static`.
pub trait Value: Clone + 'static {}

impl<T: Clone + 'static> Value for T {}
impl<T: std::hash::Hash + Eq + Clone + 'static> Key for T {}

#[cfg(feature = "bench-variants")]
struct BenchChannelHandles<V: Value> {
    sender: broadcast::Sender<V>,
    rc_sender: Rc<broadcast::Sender<V>>,
    arc_sender: Arc<broadcast::Sender<V>>,
}

#[cfg(feature = "bench-variants")]
type StreamEntry<V> = BenchChannelHandles<V>;
#[cfg(not(feature = "bench-variants"))]
type StreamEntry<V> = broadcast::Sender<V>;

// Bench scaffolding: bench-variants keeps extra handle wrappers (Rc/Arc) for
// send-variant comparison. Default build stores a plain Sender only.
type Streams<K, V> = Rc<RefCell<HashMap<K, StreamEntry<V>>>>;

#[doc(hidden)]
pub const __BENCH_BACKEND: &str = "local";

/// The main entry point for key-based async message streaming.
///
/// Use [`KeyStream::new`] to create, then call [`KeyStream::sender`] to get a sender handle.
///
/// # Example
/// ```
/// use key_stream::KeyStream;
/// use tokio;
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
///     let key_stream = KeyStream::<i32, String>::new(10);
///     let sender = key_stream.sender();
///     let mut receiver = sender.subscribe(1).await;
///     sender.send(&1, "value".to_string()).await;
///     assert_eq!(receiver.recv().await.unwrap(), "value".to_string());
/// # }
/// ```
pub struct KeyStream<K: Key, V: Value> {
    broadcast_capacity: usize,
    streams: Streams<K, V>,
}

/// Handle for sending and subscribing to messages by key.
///
/// Created via [`KeyStream::sender`].
pub struct KeySender<K: Key, V: Value> {
    streams: Streams<K, V>,
    broadcast_capacity: usize,
}

/// A receiver for messages for a specific key.
///
/// Created via [`KeySender::subscribe`].
pub struct KeyReceiver<K: Key, V: Value> {
    key: K,
    streams: Weak<RefCell<HashMap<K, StreamEntry<V>>>>,
    receiver: Option<broadcast::Receiver<V>>,
}

impl<K: Key, V: Value> KeyStream<K, V> {
    /// Create a new [`KeyStream`] with the given broadcast channel capacity per key.
    ///
    /// # Panics
    ///
    /// Panics if `broadcast_capacity == 0`, because
    /// [`tokio::sync::broadcast::channel`] requires a positive capacity.
    pub fn new(broadcast_capacity: usize) -> Self {
        let streams = Rc::new(RefCell::new(HashMap::<K, StreamEntry<V>>::new()));
        Self {
            broadcast_capacity,
            streams,
        }
    }

    /// Get a sender handle for publishing and subscribing to keys.
    pub fn sender(&self) -> KeySender<K, V> {
        KeySender::new(Rc::clone(&self.streams), self.broadcast_capacity)
    }

    /// Get the number of keys currently tracked.
    pub async fn n_keys(&self) -> usize {
        self.streams.borrow().len()
    }

    /// Get the current capacity of the keys map.
    pub async fn key_capacity(&self) -> usize {
        self.streams.borrow().capacity()
    }
}

impl<K: Key, V: Value> KeySender<K, V> {
    fn new(streams: Streams<K, V>, broadcast_capacity: usize) -> Self {
        Self {
            streams,
            broadcast_capacity,
        }
    }

    /// Send a value to all receivers subscribed to the given key.
    ///
    /// Returns the number of receivers the message was sent to, or 0 if none.
    pub async fn send(&self, key: &K, value: V) -> usize {
        self.__bench_send_clone_sender(key, value).await
    }

    #[doc(hidden)]
    pub fn __bench_sender_clone_for_key(&self, key: &K) -> Option<Rc<broadcast::Sender<V>>> {
        let streams = self.streams.borrow();
        streams.get(key).map(entry_rc_sender_clone)
    }

    #[doc(hidden)]
    pub async fn __bench_send_guard_held(&self, key: &K, value: V) -> usize {
        let streams = self.streams.borrow();
        if let Some(entry) = streams.get(key) {
            entry_sender_ref(entry).send(value).unwrap_or(0)
        } else {
            0
        }
    }

    #[doc(hidden)]
    pub async fn __bench_send_clone_sender(&self, key: &K, value: V) -> usize {
        let sender = {
            let streams = self.streams.borrow();
            streams.get(key).map(entry_sender_clone)
        };
        if let Some(sender) = sender {
            sender.send(value).unwrap_or(0)
        } else {
            0
        }
    }

    #[cfg(feature = "bench-variants")]
    #[doc(hidden)]
    pub async fn __bench_send_rc_sender_lookup(&self, key: &K, value: V) -> usize {
        let sender = {
            let streams = self.streams.borrow();
            streams.get(key).map(|entry| Rc::clone(&entry.rc_sender))
        };
        let Some(sender) = sender else {
            return 0;
        };
        sender.send(value).unwrap_or(0)
    }

    #[cfg(feature = "bench-variants")]
    #[doc(hidden)]
    pub async fn __bench_send_arc_sender(&self, key: &K, value: V) -> usize {
        let sender = {
            let streams = self.streams.borrow();
            streams.get(key).map(|entry| Arc::clone(&entry.arc_sender))
        };
        let Some(sender) = sender else {
            return 0;
        };
        sender.send(value).unwrap_or(0)
    }

    /// Subscribe to messages for the given key.
    ///
    /// Returns a [`KeyReceiver`] for receiving messages.
    pub async fn subscribe(&self, key: K) -> KeyReceiver<K, V> {
        let streams = self.streams.borrow();
        let inner = if let Some(entry) = streams.get(&key) {
            entry_subscribe(entry)
        } else {
            drop(streams);
            let mut streams = self.streams.borrow_mut();
            let entry = streams.entry(key.clone()).or_insert_with(|| {
                let (sender, _) = broadcast::channel(self.broadcast_capacity);
                make_entry(sender)
            });
            entry_subscribe(entry)
        };
        self.create_receiver(key, inner)
    }

    /// Get the number of keys currently tracked.
    pub async fn n_keys(&self) -> usize {
        self.streams.borrow().len()
    }

    /// Get the current capacity of the keys map.
    pub async fn key_capacity(&self) -> usize {
        self.streams.borrow().capacity()
    }

    fn create_receiver(&self, key: K, inner: broadcast::Receiver<V>) -> KeyReceiver<K, V> {
        KeyReceiver {
            key,
            streams: Rc::downgrade(&self.streams),
            receiver: Some(inner),
        }
    }
}

impl<K: Key, V: Value> KeyReceiver<K, V> {
    /// Receive the next message for this key, waiting asynchronously.
    /// See, tokio::sync::broadcast::Receiver::recv for more details.
    pub async fn recv(&mut self) -> Result<V, broadcast::error::RecvError> {
        self.receiver
            .as_mut()
            .expect("receiver missing")
            .recv()
            .await
    }

    /// Try to receive the next message for this key without waiting.
    /// See, tokio::sync::broadcast::Receiver::try_recv for more details.
    pub fn try_recv(&mut self) -> Result<V, broadcast::error::TryRecvError> {
        self.receiver.as_mut().expect("receiver missing").try_recv()
    }

    /// Receive the next message for this key, blocking the current thread.
    /// Only use in synchronous contexts.
    /// See, tokio::sync::broadcast::Receiver::blocking_recv for more details.
    pub fn blocking_recv(&mut self) -> Result<V, broadcast::error::RecvError> {
        self.receiver
            .as_mut()
            .expect("receiver missing")
            .blocking_recv()
    }

    /// Consume this receiver and convert it into a stream of messages for this key.
    ///
    /// The stream ends when the channel is closed (all senders and the
    /// [`KeyStream`] have been dropped). Missed messages are reported as
    /// [`RecvError::Lagged`] items.
    pub fn into_stream(self) -> impl Stream<Item = Result<V, RecvError>> {
        futures::stream::unfold(self, |mut receiver| async {
            match receiver.recv().await {
                Err(RecvError::Closed) => None,
                item => Some((item, receiver)),
            }
        })
    }
}

impl<K: Key, V: Value> Clone for KeySender<K, V> {
    fn clone(&self) -> Self {
        Self {
            streams: Rc::clone(&self.streams),
            broadcast_capacity: self.broadcast_capacity,
        }
    }
}

impl<K: Key, V: Value> Drop for KeyReceiver<K, V> {
    fn drop(&mut self) {
        // Drop the receiver before borrowing the map: receiver drop may drain buffered
        // slots and run value destructors, which may re-enter the stream.
        let Some(receiver) = self.receiver.take() else {
            return;
        };
        drop(receiver);

        let Some(streams_ref) = self.streams.upgrade() else {
            return;
        };
        let mut streams = streams_ref.borrow_mut();
        if let Some(entry) = streams.get(&self.key)
            && entry_receiver_count(entry) == 0
        {
            streams.remove(&self.key);
            optimize_dict_mem(&mut streams);
        }
    }
}

#[cfg(feature = "bench-variants")]
fn make_entry<V: Value>(sender: broadcast::Sender<V>) -> StreamEntry<V> {
    BenchChannelHandles {
        sender: sender.clone(),
        rc_sender: Rc::new(sender.clone()),
        arc_sender: Arc::new(sender),
    }
}

#[cfg(not(feature = "bench-variants"))]
fn make_entry<V: Value>(sender: broadcast::Sender<V>) -> StreamEntry<V> {
    sender
}

#[cfg(feature = "bench-variants")]
fn entry_sender_ref<V: Value>(entry: &StreamEntry<V>) -> &broadcast::Sender<V> {
    &entry.sender
}

#[cfg(not(feature = "bench-variants"))]
fn entry_sender_ref<V: Value>(entry: &StreamEntry<V>) -> &broadcast::Sender<V> {
    entry
}

fn entry_sender_clone<V: Value>(entry: &StreamEntry<V>) -> broadcast::Sender<V> {
    entry_sender_ref(entry).clone()
}

#[cfg(feature = "bench-variants")]
fn entry_rc_sender_clone<V: Value>(entry: &StreamEntry<V>) -> Rc<broadcast::Sender<V>> {
    Rc::clone(&entry.rc_sender)
}

#[cfg(not(feature = "bench-variants"))]
fn entry_rc_sender_clone<V: Value>(entry: &StreamEntry<V>) -> Rc<broadcast::Sender<V>> {
    Rc::new(entry.clone())
}

fn entry_subscribe<V: Value>(entry: &StreamEntry<V>) -> broadcast::Receiver<V> {
    entry_sender_ref(entry).subscribe()
}

fn entry_receiver_count<V: Value>(entry: &StreamEntry<V>) -> usize {
    entry_sender_ref(entry).receiver_count()
}

fn optimize_dict_mem<K: Key, V: Value>(streams: &mut HashMap<K, StreamEntry<V>>) {
    // If the number of keys is less than half the capacity and the capacity is big enough,
    // shrink the capacity to save memory
    let cap = streams.capacity() >> 1;
    let len = streams.len();
    if cap > 64 && len < cap {
        streams.shrink_to(cap);
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::pin::Pin;
    use std::rc::Rc;
    use tokio::time::{Duration, timeout};

    use super::*;

    #[tokio::test(flavor = "current_thread")]
    async fn test_recv() {
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender();
        let mut receiver = sender.subscribe("1".to_string()).await;
        assert_eq!(key_stream.n_keys().await, 1);
        sender.send(&"1".to_string(), "value".to_string()).await;
        assert_eq!(receiver.recv().await.unwrap(), "value".to_string());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_no_receiver() {
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender();
        let result = sender.send(&"1".to_string(), "value".to_string()).await;
        assert_eq!(result, 0);
        assert_eq!(key_stream.n_keys().await, 0);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_send_after_drop() {
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender();
        let receiver = sender.subscribe("1".to_string()).await;
        assert_eq!(key_stream.n_keys().await, 1);
        drop(receiver);
        let result = sender.send(&"1".to_string(), "value".to_string()).await;
        assert_eq!(result, 0);
        assert_eq!(key_stream.n_keys().await, 0);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_send_after_drop_reclaims_key_immediately() {
        // All receivers are gone, so send() must report 0 receivers and the key
        // must be reclaimed synchronously.
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender();
        let receiver = sender.subscribe("1".to_string()).await;
        drop(receiver);
        let result = sender.send(&"1".to_string(), "value".to_string()).await;
        assert_eq!(result, 0);
        assert_eq!(key_stream.n_keys().await, 0);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_lagged() {
        let key_stream = KeyStream::<String, String>::new(1);
        let sender = key_stream.sender();
        let mut receiver = sender.subscribe("1".to_string()).await;
        let result = sender.send(&"1".to_string(), "value1".to_string()).await;
        assert_eq!(result, 1);
        let result = sender.send(&"1".to_string(), "value2".to_string()).await;
        assert_eq!(result, 1);
        assert_eq!(
            receiver.recv().await,
            Err(broadcast::error::RecvError::Lagged(1))
        );
        assert_eq!(receiver.recv().await.unwrap(), "value2".to_string());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_recv_struct() {
        #[derive(Clone, Debug)]
        struct MyStruct {
            field1: String,
            field2: i32,
        }
        let key_stream = KeyStream::<String, MyStruct>::new(10);
        let sender = key_stream.sender();
        let mut receiver = sender.subscribe("1".to_string()).await;
        assert_eq!(key_stream.n_keys().await, 1);
        sender
            .send(
                &"1".to_string(),
                MyStruct {
                    field1: "value".to_string(),
                    field2: 42,
                },
            )
            .await;
        let received = receiver.recv().await.unwrap();
        assert_eq!(received.field1, "value".to_string());
        assert_eq!(received.field2, 42);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_recv_rc_struct() {
        #[derive(Debug)]
        struct MyStruct {
            field1: String,
            field2: i32,
        }
        let key_stream = KeyStream::<String, Rc<MyStruct>>::new(10);
        let sender = key_stream.sender();
        let mut receiver = sender.subscribe("1".to_string()).await;
        assert_eq!(key_stream.n_keys().await, 1);
        sender
            .send(
                &"1".to_string(),
                Rc::new(MyStruct {
                    field1: "value".to_string(),
                    field2: 42,
                }),
            )
            .await;
        let received = receiver.recv().await.unwrap();

        assert_eq!(received.field1, "value".to_string());
        assert_eq!(received.field2, 42);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_messages_broadcasted() {
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender();
        let mut receiver1 = sender.subscribe("1".to_string()).await;
        let mut receiver2 = sender.subscribe("1".to_string()).await;
        assert_eq!(key_stream.n_keys().await, 1);
        sender.send(&"1".to_string(), "value".to_string()).await;
        assert_eq!(receiver1.recv().await.unwrap(), "value".to_string());
        assert_eq!(receiver2.recv().await.unwrap(), "value".to_string());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_key_filter() {
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender();
        let mut receiver1 = sender.subscribe("1".to_string()).await;
        let mut receiver2 = sender.subscribe("2".to_string()).await;
        assert_eq!(key_stream.n_keys().await, 2);
        sender.send(&"1".to_string(), "value1".to_string()).await;
        sender.send(&"2".to_string(), "value2".to_string()).await;
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

    #[tokio::test(flavor = "current_thread")]
    async fn test_key_drop() {
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender();
        let receiver = sender.subscribe("1".to_string()).await;
        assert_eq!(key_stream.n_keys().await, 1);
        sender.send(&"1".to_string(), "value".to_string()).await;
        drop(receiver);
        assert_eq!(key_stream.n_keys().await, 0);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_key_non_dropped_if_other_receiver_exists() {
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender();
        let receiver1 = sender.subscribe("1".to_string()).await;
        let _receiver2 = sender.subscribe("1".to_string()).await;
        assert_eq!(key_stream.n_keys().await, 1);
        sender.send(&"1".to_string(), "value".to_string()).await;
        drop(receiver1);
        assert_eq!(key_stream.n_keys().await, 1);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_two_clients() {
        let key_stream = KeyStream::<String, String>::new(10);
        let sender1 = key_stream.sender();
        let sender2 = key_stream.sender();
        let mut receiver1 = sender1.subscribe("1".to_string()).await;
        let mut receiver2 = sender2.subscribe("1".to_string()).await;

        assert_eq!(key_stream.n_keys().await, 1);

        sender1.send(&"1".to_string(), "value".to_string()).await;

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

        assert_eq!(key_stream.n_keys().await, 1);

        sender1.send(&"1".to_string(), "value".to_string()).await;

        assert_eq!(receiver1.recv().await.unwrap(), "value".to_string());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_reconnect() {
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender();
        let mut receiver1 = sender.subscribe("1".to_string()).await;
        assert_eq!(key_stream.n_keys().await, 1);
        sender.send(&"1".to_string(), "value".to_string()).await;
        assert_eq!(receiver1.recv().await.unwrap(), "value".to_string());
        drop(receiver1);
        assert_eq!(key_stream.n_keys().await, 0);
        let mut receiver2 = sender.subscribe("1".to_string()).await;
        assert_eq!(key_stream.n_keys().await, 1);
        sender.send(&"1".to_string(), "value2".to_string()).await;
        assert_eq!(receiver2.recv().await.unwrap(), "value2".to_string());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_resubscribe_after_drop_recreates_key() {
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender();
        let receiver1 = sender.subscribe("1".to_string()).await;
        assert_eq!(key_stream.n_keys().await, 1);

        drop(receiver1);
        assert_eq!(key_stream.n_keys().await, 0);

        let mut receiver2 = sender.subscribe("1".to_string()).await;
        assert_eq!(key_stream.n_keys().await, 1);

        let result = sender.send(&"1".to_string(), "value".to_string()).await;
        assert_eq!(result, 1);
        assert_eq!(receiver2.recv().await.unwrap(), "value".to_string());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_shrink_dict() {
        use futures::future::join_all;

        let key_stream = KeyStream::<i32, String>::new(1);
        let sender = key_stream.sender();
        let tasks = (0..500).map(|i| {
            let sender = sender.clone();
            async move { sender.subscribe(i).await }
        });
        // keep the receivers alive; dropping them is what triggers cleanup
        let mut subs = join_all(tasks).await;
        assert_eq!(key_stream.n_keys().await, 500);
        subs.drain(0..400);
        assert_eq!(key_stream.n_keys().await, 100);
        assert!(key_stream.key_capacity().await < 500);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_drop_sender() {
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender();
        let mut receiver = sender.subscribe("1".to_string()).await;
        assert_eq!(key_stream.n_keys().await, 1);
        sender.send(&"1".to_string(), "value".to_string()).await;
        drop(sender);
        assert_eq!(receiver.recv().await.unwrap(), "value".to_string());
        assert_eq!(
            receiver.try_recv(),
            Err(broadcast::error::TryRecvError::Empty)
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_drop_stream() {
        // This test ensures that dropping the KeyStream doesn't cause any panics
        // actual data may vary
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender();
        drop(key_stream);
        let mut receiver = sender.subscribe("1".to_string()).await;
        sender.send(&"1".to_string(), "value".to_string()).await;
        assert_eq!(receiver.recv().await.unwrap(), "value".to_string());
        assert_eq!(
            receiver.try_recv(),
            Err(broadcast::error::TryRecvError::Empty)
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_len_and_capacity() {
        let key_stream = KeyStream::<String, String>::new(10);
        let sender1 = key_stream.sender();
        let sender2 = key_stream.sender();
        assert_eq!(key_stream.n_keys().await, 0);
        assert_eq!(sender1.n_keys().await, 0);
        assert_eq!(sender2.n_keys().await, 0);
        assert_eq!(
            key_stream.key_capacity().await,
            sender1.key_capacity().await
        );
        assert_eq!(
            key_stream.key_capacity().await,
            sender2.key_capacity().await
        );
        let _receiver1 = sender1.subscribe("1".to_string()).await;
        let _receiver2 = sender2.subscribe("2".to_string()).await;
        let _receiver3 = sender2.subscribe("3".to_string()).await;
        assert_eq!(key_stream.n_keys().await, 3);
        assert_eq!(sender1.n_keys().await, 3);
        assert_eq!(sender2.n_keys().await, 3);
        assert_eq!(
            key_stream.key_capacity().await,
            sender1.key_capacity().await
        );
        assert_eq!(
            key_stream.key_capacity().await,
            sender2.key_capacity().await
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_stream() {
        use futures::StreamExt;
        type WatchStream =
            Pin<Box<dyn futures::Stream<Item = Result<String, RecvError>> + 'static>>;
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender();
        let receiver = sender.subscribe("1".to_string()).await;
        sender.send(&"1".to_string(), "value1".to_string()).await;
        let stream = receiver.into_stream();
        let stream = Box::pin(stream) as WatchStream;
        sender.send(&"1".to_string(), "value2".to_string()).await;
        let msg = timeout(Duration::from_secs(1), stream.take(2).collect::<Vec<_>>()).await;
        let msg = msg.expect("timeout");
        assert_eq!(
            msg,
            vec![Ok("value1".to_string()), Ok("value2".to_string())]
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_stream_ends_when_channel_closes() {
        use futures::StreamExt;
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender();
        let receiver = sender.subscribe("1".to_string()).await;
        sender.send(&"1".to_string(), "value".to_string()).await;
        // Dropping the sender and the KeyStream drops the broadcast sender,
        // closing the channel: the stream must yield the buffered message and
        // then terminate instead of repeating Err(Closed) forever.
        drop(sender);
        drop(key_stream);
        let stream = receiver.into_stream();
        let msg = timeout(Duration::from_secs(1), stream.collect::<Vec<_>>()).await;
        let msg = msg.expect("stream did not terminate after close");
        assert_eq!(msg, vec![Ok("value".to_string())]);
    }

    #[derive(Clone)]
    struct ReentrantDrop {
        armed: Rc<Cell<bool>>,
        on_drop: Rc<dyn Fn()>,
    }

    impl Drop for ReentrantDrop {
        fn drop(&mut self) {
            if self.armed.replace(false) {
                (self.on_drop)();
            }
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_send_clone_sender_reentrant_drop_no_panic() {
        let key_stream = KeyStream::<u64, ReentrantDrop>::new(1);
        let sender = key_stream.sender();
        let mut lagging_receiver = sender.subscribe(1).await;

        let armed = Rc::new(Cell::new(false));
        let streams = Rc::clone(&key_stream.streams);
        let on_drop: Rc<dyn Fn()> = Rc::new(move || {
            let _guard = streams.borrow_mut();
        });

        assert_eq!(
            sender
                .send(
                    &1,
                    ReentrantDrop {
                        armed: Rc::clone(&armed),
                        on_drop: Rc::clone(&on_drop),
                    },
                )
                .await,
            1
        );

        std::hint::black_box(&mut lagging_receiver);
        armed.set(true);

        assert_eq!(
            sender
                .send(
                    &1,
                    ReentrantDrop {
                        armed: Rc::clone(&armed),
                        on_drop: Rc::clone(&on_drop),
                    },
                )
                .await,
            1
        );
    }

    #[cfg(feature = "bench-variants")]
    #[tokio::test(flavor = "current_thread")]
    async fn test_bench_send_arc_sender_delivers_to_receivers() {
        let key_stream = KeyStream::<u64, String>::new(8);
        let sender = key_stream.sender();
        let mut receiver = sender.subscribe(1).await;
        assert_eq!(
            sender
                .__bench_send_arc_sender(&1, "value".to_string())
                .await,
            1
        );
        assert_eq!(receiver.recv().await.unwrap(), "value".to_string());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_receiver_drop_take_first_prevents_reentrant_borrow_panic() {
        // Regression: this specifically catches reordering inside KeyReceiver::drop.
        // If map borrow happens before dropping the inner Receiver, dropping unread
        // buffered values can re-enter the stream and panic with "already borrowed".
        let key_stream = KeyStream::<u64, ReentrantDrop>::new(1);
        let sender = key_stream.sender();
        let receiver = sender.subscribe(1).await;

        let armed = Rc::new(Cell::new(true));
        let streams = Rc::clone(&key_stream.streams);
        let on_drop: Rc<dyn Fn()> = Rc::new(move || {
            let _guard = streams.borrow_mut();
        });

        assert_eq!(
            sender
                .send(
                    &1,
                    ReentrantDrop {
                        armed: Rc::clone(&armed),
                        on_drop,
                    },
                )
                .await,
            1
        );

        // Keep the value unread so dropping the receiver drains and drops it.
        drop(receiver);
        assert_eq!(key_stream.n_keys().await, 0);
    }

    #[tokio::test(flavor = "current_thread")]
    #[should_panic(expected = "already borrowed")]
    async fn test_send_guard_held_reentrant_drop_panics_refcell() {
        let key_stream = KeyStream::<u64, ReentrantDrop>::new(1);
        let sender = key_stream.sender();
        let mut lagging_receiver = sender.subscribe(1).await;

        let armed = Rc::new(Cell::new(false));
        let streams = Rc::clone(&key_stream.streams);
        let on_drop: Rc<dyn Fn()> = Rc::new(move || {
            // Re-enter map mutation while send() still holds an immutable borrow.
            let _guard = streams.borrow_mut();
        });

        assert_eq!(
            sender
                .__bench_send_guard_held(
                    &1,
                    ReentrantDrop {
                        armed: Rc::clone(&armed),
                        on_drop: Rc::clone(&on_drop),
                    },
                )
                .await,
            1
        );

        // Keep the first value unread so the next send evicts and drops it.
        std::hint::black_box(&mut lagging_receiver);

        armed.set(true);

        let _ = sender
            .__bench_send_guard_held(
                &1,
                ReentrantDrop {
                    armed: Rc::clone(&armed),
                    on_drop: Rc::clone(&on_drop),
                },
            )
            .await;
    }
}
