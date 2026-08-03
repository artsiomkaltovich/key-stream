//! Async key-based message streaming library with two backends and no default —
//! `key_stream::KeyStream` deliberately does not resolve, so pick one explicitly:
//!
//! - [`shared`] — `Arc<RwLock<_>>`, `Send + Sync`, requires `V: Send`. **Start here**
//!   unless you know the stream never leaves one thread.
//! - [`local`] — `Rc<RefCell<_>>`, not `Send` by design, accepts non-`Send` values such
//!   as `V = Rc<T>`, and pays no atomic refcounting.
//!
//! The broadcast channel is overwriting:
//! - `send` never blocks and never reports "full";
//! - if a receiver lags, it sees `RecvError::Lagged(n)` on `recv`;
//! - `send` returns `0` when there are no receivers (including missing keys).
//!
//! # Examples
//!
//! `shared` — the usual choice:
//!
//! ```
//! use key_stream::shared::KeyStream;
//! # #[tokio::main(flavor = "multi_thread")]
//! # async fn main() {
//! let key_stream = KeyStream::<i32, String>::new(10);
//! let sender = key_stream.sender();
//! let mut receiver = sender.subscribe(1);
//! sender.send(&1, "value".to_string());
//! assert_eq!(receiver.recv().await.unwrap(), "value".to_string());
//! # }
//! ```
//!
//! `local` — same API, single-threaded, and able to carry `Rc` values:
//!
//! ```
//! use key_stream::local::KeyStream;
//! use std::rc::Rc;
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() {
//! let key_stream = KeyStream::<i32, Rc<String>>::new(10);
//! let sender = key_stream.sender();
//! let mut receiver = sender.subscribe(1);
//! sender.send(&1, Rc::new("value".to_string()));
//! assert_eq!(*receiver.recv().await.unwrap(), "value".to_string());
//! # }
//! ```

mod core;
pub mod local;
pub mod shared;
