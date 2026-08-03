//! Multi-threaded backend.
//!
//! Uses `Arc<RwLock<...>>` for the map and `Arc<Sender<_>>` handles.
//! Values must be `Send`.
//!
//! `send` is non-blocking and returns:
//! - `N > 0` when delivered to `N` receivers;
//! - `0` when the key is absent or has no live receivers.
//!
//! # Example
//! ```
//! use key_stream::shared::KeyStream;
//! # #[tokio::main(flavor = "multi_thread")]
//! # async fn main() {
//! let key_stream = KeyStream::<String, String>::new(10);
//! let sender = key_stream.sender();
//! let mut receiver = sender.subscribe("k".to_string());
//! sender.send(&"k".to_string(), "v".to_string());
//! assert_eq!(receiver.recv().await.unwrap(), "v".to_string());
//! # }
//! ```

use crate::core::{Backend, KeyReceiverCore, KeySenderCore, KeyStreamCore};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak};
use tokio::sync::broadcast;

pub trait Key: Hash + Eq + Clone + Send + Sync + 'static {}
pub trait Value: Clone + Send + 'static {}

impl<T: Hash + Eq + Clone + Send + Sync + 'static> Key for T {}
impl<T: Clone + Send + 'static> Value for T {}

type Streams<K, V> = Arc<RwLock<HashMap<K, Arc<broadcast::Sender<V>>>>>;

impl<K: Key, V: Value> Backend<K, V> for Streams<K, V> {
    type Handle = Arc<broadcast::Sender<V>>;
    type WeakSelf = Weak<RwLock<HashMap<K, Arc<broadcast::Sender<V>>>>>;
    type ReadGuard<'a>
        = RwLockReadGuard<'a, HashMap<K, Self::Handle>>
    where
        Self: 'a;
    type WriteGuard<'a>
        = RwLockWriteGuard<'a, HashMap<K, Self::Handle>>
    where
        Self: 'a;

    fn new() -> Self {
        Arc::new(RwLock::new(HashMap::new()))
    }

    fn wrap(sender: broadcast::Sender<V>) -> Self::Handle {
        Arc::new(sender)
    }

    fn read(&self) -> Self::ReadGuard<'_> {
        RwLock::read(self).unwrap_or_else(PoisonError::into_inner)
    }

    fn write(&self) -> Self::WriteGuard<'_> {
        RwLock::write(self).unwrap_or_else(PoisonError::into_inner)
    }

    fn downgrade(&self) -> Self::WeakSelf {
        Arc::downgrade(self)
    }

    fn upgrade(weak: &Self::WeakSelf) -> Option<Self> {
        weak.upgrade()
    }
}

pub type KeyStream<K, V> = KeyStreamCore<Streams<K, V>, K, V>;
pub type KeySender<K, V> = KeySenderCore<Streams<K, V>, K, V>;
pub type KeyReceiver<K, V> = KeyReceiverCore<Streams<K, V>, K, V>;
