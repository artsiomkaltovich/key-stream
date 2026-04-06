use std::hash::Hash;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use tokio::sync::{RwLock, broadcast};

pub trait Key: Hash + Eq + Clone + Send + Sync + 'static {}
pub trait Value: Clone + Send + 'static {}
impl<T: Clone + Send + 'static> Value for T {}
impl<T: std::hash::Hash + Eq + Clone + Send + Sync + 'static> Key for T {}

pub struct KeyStream<K: Key, V: Value> {
    broadcast_capacity: usize,
    streams: Arc<RwLock<HashMap<K, broadcast::Sender<V>>>>,
    sender: UnboundedSender<K>,
    drop_keys_task: Option<tokio::task::JoinHandle<()>>,
}

pub struct KeySender<K: Key, V: Value> {
    streams: Arc<RwLock<HashMap<K, broadcast::Sender<V>>>>,
    broadcast_capacity: usize,
    drop_notify: UnboundedSender<K>,
}

pub struct KeyReceiver<K: Key, V: Value> {
    key: K,
    receiver: broadcast::Receiver<V>,
    on_drop: UnboundedSender<K>,
}

impl<K: Key, V: Value> KeyStream<K, V> {
    pub fn new(broadcast_capacity: usize) -> Self {
        let (sender, receiver) = unbounded_channel::<K>();
        let streams = Arc::new(RwLock::new(HashMap::<K, broadcast::Sender<V>>::new()));
        let on_drop_task = tokio::spawn(cleanup_keys(receiver, Arc::clone(&streams)));
        Self {
            broadcast_capacity,
            streams,
            sender,
            drop_keys_task: Some(on_drop_task),
        }
    }

    pub async fn sender(&self) -> KeySender<K, V> {
        KeySender::new(
            Arc::clone(&self.streams),
            self.broadcast_capacity,
            self.sender.clone(),
        )
    }
}

impl<K: Key, V: Value> KeySender<K, V> {
    fn new(
        streams: Arc<RwLock<HashMap<K, broadcast::Sender<V>>>>,
        broadcast_capacity: usize,
        sender: UnboundedSender<K>,
    ) -> Self {
        Self {
            streams,
            broadcast_capacity,
            drop_notify: sender,
        }
    }

    pub async fn send(&self, key: &K, value: V) -> Result<usize, broadcast::error::SendError<V>> {
        let streams = self.streams.read().await;
        if let Some(sender) = streams.get(key) {
            sender.send(value)
        } else {
            Ok(0)
        }
    }

    pub async fn subscribe(&self, key: K) -> KeyReceiver<K, V> {
        let streams = self.streams.read().await;
        if let Some(sender) = streams.get(&key) {
            KeyReceiver {
                key,
                receiver: sender.subscribe(),
                on_drop: self.drop_notify.clone(),
            }
        } else {
            drop(streams);
            let mut streams = self.streams.write().await;
            let sender = streams.entry(key.clone()).or_insert_with(|| {
                let (sender, _) = broadcast::channel(self.broadcast_capacity);
                sender
            });
            KeyReceiver {
                key,
                receiver: sender.subscribe(),
                on_drop: self.drop_notify.clone(),
            }
        }
    }
}

impl<K: Key, V: Value> KeyReceiver<K, V> {
    pub async fn recv(&mut self) -> Result<V, broadcast::error::RecvError> {
        self.receiver.recv().await
    }

    pub async fn try_recv(&mut self) -> Result<V, broadcast::error::TryRecvError> {
        self.receiver.try_recv()
    }

    pub fn blocking_recv(&mut self) -> Result<V, broadcast::error::RecvError> {
        self.receiver.blocking_recv()
    }
}

impl<K: Key, V: Value> Clone for KeySender<K, V> {
    fn clone(&self) -> Self {
        Self {
            streams: Arc::clone(&self.streams),
            broadcast_capacity: self.broadcast_capacity,
            drop_notify: self.drop_notify.clone(),
        }
    }
}

impl<K: Key, V: Value> Drop for KeyReceiver<K, V> {
    fn drop(&mut self) {
        let Ok(_) = self.on_drop.send(self.key.clone()) else {
            dbg!("Failed to send drop notification");
            return;
        };
    }
}

impl<K: Key, V: Value> Drop for KeyStream<K, V> {
    fn drop(&mut self) {
        if let Some(task) = self.drop_keys_task.take() {
            task.abort();
        }
    }
}

async fn cleanup_keys<K: Key, V: Value>(
    mut receiver: tokio::sync::mpsc::UnboundedReceiver<K>,
    streams: Arc<RwLock<HashMap<K, broadcast::Sender<V>>>>,
) {
    while let Some(key) = receiver.recv().await {
        let mut streams = streams.write().await;
        if let Some(sender) = streams.get(&key)
            && sender.receiver_count() == 0
        {
            streams.remove(&key);
            optimize_dict_mem(streams);
        }
    }
}

fn optimize_dict_mem<K: Key, V: Value>(
    mut streams: tokio::sync::RwLockWriteGuard<'_, HashMap<K, broadcast::Sender<V>>>,
) {
    // If the number of keys is less than half the capacity and the capacity is big enough,
    // shrink the capacity to save memory
    let cap = streams.capacity() << 1;
    let len = streams.len();
    if cap > 64 && len < cap {
        streams.shrink_to(cap);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_recv() {
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender().await;
        let mut receiver = sender.subscribe("1".to_string()).await;
        assert_eq!(key_stream.streams.read().await.len(), 1);
        sender
            .send(&"1".to_string(), "value".to_string())
            .await
            .unwrap();
        assert_eq!(receiver.recv().await.unwrap(), "value".to_string());
    }

    #[tokio::test]
    async fn test_no_receiver() {
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender().await;
        let receiver = sender.subscribe("1".to_string()).await;
        assert_eq!(key_stream.streams.read().await.len(), 1);
        drop(receiver);
        // give the on_drop task a chance to run
        tokio::task::yield_now().await;
        let result = sender
            .send(&"1".to_string(), "value".to_string())
            .await
            .unwrap();
        assert_eq!(result, 0);
        assert_eq!(key_stream.streams.read().await.len(), 0);
    }

    #[tokio::test]
    async fn test_recv_struct() {
        #[derive(Clone, Debug)]
        struct MyStruct {
            field1: String,
            field2: i32,
        }
        let key_stream = KeyStream::<String, MyStruct>::new(10);
        let sender = key_stream.sender().await;
        let mut receiver = sender.subscribe("1".to_string()).await;
        assert_eq!(key_stream.streams.read().await.len(), 1);
        sender
            .send(
                &"1".to_string(),
                MyStruct {
                    field1: "value".to_string(),
                    field2: 42,
                },
            )
            .await
            .unwrap();
        let received = receiver.recv().await.unwrap();
        assert_eq!(received.field1, "value".to_string());
        assert_eq!(received.field2, 42);
    }

    #[tokio::test]
    async fn test_recv_arc_struct() {
        #[derive(Debug)]
        struct MyStruct {
            field1: String,
            field2: i32,
        }
        let key_stream = KeyStream::<String, Arc<MyStruct>>::new(10);
        let sender = key_stream.sender().await;
        let mut receiver = sender.subscribe("1".to_string()).await;
        assert_eq!(key_stream.streams.read().await.len(), 1);
        sender
            .send(
                &"1".to_string(),
                Arc::new(MyStruct {
                    field1: "value".to_string(),
                    field2: 42,
                }),
            )
            .await
            .unwrap();
        let received = receiver.recv().await.unwrap();

        assert_eq!(received.field1, "value".to_string());
        assert_eq!(received.field2, 42);
    }

    #[tokio::test]
    async fn test_messages_broadcasted() {
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender().await;
        let mut receiver1 = sender.subscribe("1".to_string()).await;
        let mut receiver2 = sender.subscribe("1".to_string()).await;
        assert_eq!(key_stream.streams.read().await.len(), 1);
        sender
            .send(&"1".to_string(), "value".to_string())
            .await
            .unwrap();
        assert_eq!(receiver1.recv().await.unwrap(), "value".to_string());
        assert_eq!(receiver2.recv().await.unwrap(), "value".to_string());
    }

    #[tokio::test]
    async fn test_key_filter() {
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender().await;
        let mut receiver1 = sender.subscribe("1".to_string()).await;
        let mut receiver2 = sender.subscribe("2".to_string()).await;
        assert_eq!(key_stream.streams.read().await.len(), 2);
        sender
            .send(&"1".to_string(), "value1".to_string())
            .await
            .unwrap();
        sender
            .send(&"2".to_string(), "value2".to_string())
            .await
            .unwrap();
        assert_eq!(receiver1.recv().await.unwrap(), "value1".to_string());
        assert_eq!(receiver2.recv().await.unwrap(), "value2".to_string());
        assert_eq!(
            receiver1.try_recv().await,
            Err(broadcast::error::TryRecvError::Empty)
        );
        assert_eq!(
            receiver2.try_recv().await,
            Err(broadcast::error::TryRecvError::Empty)
        );
    }

    #[tokio::test]
    async fn test_key_drop() {
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender().await;
        let receiver = sender.subscribe("1".to_string()).await;
        assert_eq!(key_stream.streams.read().await.len(), 1);
        sender
            .send(&"1".to_string(), "value".to_string())
            .await
            .unwrap();
        drop(receiver);
        // give the on_drop task a chance to run
        tokio::task::yield_now().await;
        assert_eq!(key_stream.streams.read().await.len(), 0);
    }

    #[tokio::test]
    async fn test_key_non_dropped_if_other_receiver_exists() {
        let key_stream = KeyStream::<String, String>::new(10);
        let sender = key_stream.sender().await;
        let receiver1 = sender.subscribe("1".to_string()).await;
        let _receiver2 = sender.subscribe("1".to_string()).await;
        assert_eq!(key_stream.streams.read().await.len(), 1);
        sender
            .send(&"1".to_string(), "value".to_string())
            .await
            .unwrap();
        drop(receiver1);
        // give the on_drop task a chance to run
        tokio::task::yield_now().await;
        assert_eq!(key_stream.streams.read().await.len(), 1);
    }

    #[tokio::test]
    async fn test_two_clients() {
        let key_stream = KeyStream::<String, String>::new(10);
        let sender1 = key_stream.sender().await;
        let sender2 = key_stream.sender().await;
        let mut receiver1 = sender1.subscribe("1".to_string()).await;
        let mut receiver2 = sender2.subscribe("1".to_string()).await;

        assert_eq!(key_stream.streams.read().await.len(), 1);

        sender1
            .send(&"1".to_string(), "value".to_string())
            .await
            .unwrap();

        assert_eq!(receiver1.recv().await.unwrap(), "value".to_string());
        assert_eq!(receiver2.recv().await.unwrap(), "value".to_string());
        assert_eq!(
            receiver1.try_recv().await,
            Err(broadcast::error::TryRecvError::Empty)
        );
        assert_eq!(
            receiver2.try_recv().await,
            Err(broadcast::error::TryRecvError::Empty)
        );

        drop(sender2);
        // give the on_drop task a chance to run
        tokio::task::yield_now().await;

        assert_eq!(key_stream.streams.read().await.len(), 1);

        sender1
            .send(&"1".to_string(), "value".to_string())
            .await
            .unwrap();

        assert_eq!(receiver1.recv().await.unwrap(), "value".to_string());
    }
}
