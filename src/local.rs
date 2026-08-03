//! Single-threaded backend.
//!
//! Uses `Rc<RefCell<...>>` for the map and `Rc<Sender<_>>` handles, so values do not
//! need to be `Send`.
//!
//! `send` is non-blocking and returns:
//! - `N > 0` when delivered to `N` receivers;
//! - `0` when the key is absent or has no live receivers.
//!
//! # Example
//! ```
//! use key_stream::local::KeyStream;
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() {
//! let key_stream = KeyStream::<String, String>::new(10);
//! let sender = key_stream.sender();
//! let mut receiver = sender.subscribe("k".to_string());
//! sender.send(&"k".to_string(), "v".to_string());
//! assert_eq!(receiver.recv().await.unwrap(), "v".to_string());
//! # }
//! ```

use crate::core::{Backend, KeyReceiverCore, KeySenderCore, KeyStreamCore};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};
use tokio::sync::broadcast;

pub trait Key: Hash + Eq + Clone + 'static {}
pub trait Value: Clone + 'static {}

impl<T: Hash + Eq + Clone + 'static> Key for T {}
impl<T: Clone + 'static> Value for T {}

type Streams<K, V> = Rc<RefCell<HashMap<K, Rc<broadcast::Sender<V>>>>>;

impl<K: Key, V: Value> Backend<K, V> for Streams<K, V> {
    type Handle = Rc<broadcast::Sender<V>>;
    type WeakSelf = Weak<RefCell<HashMap<K, Rc<broadcast::Sender<V>>>>>;
    type ReadGuard<'a>
        = Ref<'a, HashMap<K, Self::Handle>>
    where
        Self: 'a;
    type WriteGuard<'a>
        = RefMut<'a, HashMap<K, Self::Handle>>
    where
        Self: 'a;

    fn new() -> Self {
        Rc::new(RefCell::new(HashMap::new()))
    }

    fn wrap(sender: broadcast::Sender<V>) -> Self::Handle {
        Rc::new(sender)
    }

    fn read(&self) -> Self::ReadGuard<'_> {
        self.borrow()
    }

    fn write(&self) -> Self::WriteGuard<'_> {
        self.borrow_mut()
    }

    fn downgrade(&self) -> Self::WeakSelf {
        Rc::downgrade(self)
    }

    fn upgrade(weak: &Self::WeakSelf) -> Option<Self> {
        weak.upgrade()
    }
}

pub type KeyStream<K, V> = KeyStreamCore<Streams<K, V>, K, V>;
pub type KeySender<K, V> = KeySenderCore<Streams<K, V>, K, V>;
pub type KeyReceiver<K, V> = KeyReceiverCore<Streams<K, V>, K, V>;
