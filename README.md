# Key Stream

I needed a small async library to send and receive messages within a single process by key, but I couldn't find one that met all of the following criteria:

- Asynchronous
- Allows subscription and publishing by key
- Avoids copying every message to every receiver and then filter
- Lightweight

So, I decided to create my own.

## What This Library Is Not

- *Not for networking*: messages are sent only within a single process.
- *No persistence*: if there are no subscribers, messages are not saved. It is expected that you obtain a snapshot elsewhere and subscribe only for updates.
If no subscribers exist, it means no one is interested at the moment and a full snapshot can be retrieved later.
It also means that the key and associated sender are deleted, so memory will not grow indefinitely.

## Choosing a Backend

There is no default. Pick one explicitly — `key_stream::KeyStream` does not resolve.

|  | `key_stream::shared` | `key_stream::local` |
|---|---|---|
| **Reach for it when** | the default choice, and the successor to the old crate-root `KeyStream` | the stream provably never crosses a thread |
| Tokio runtime flavor | any — `multi_thread` or `current_thread` | any, but see the spawn rule below |
| Spawning tasks that hold it | `tokio::spawn` | `tokio::task::spawn_local` inside a [`LocalSet`], or plain `join_all` on one task |
| `KeyStream` / `KeySender` | `Send + Sync` | neither, **by design** |
| Map cell | `Arc<RwLock<HashMap<..>>>` | `Rc<RefCell<HashMap<..>>>` |
| Channel handle | `Arc<broadcast::Sender<V>>` | `Rc<broadcast::Sender<V>>` |
| `Key` bound | `Hash + Eq + Clone + Send + Sync + 'static` | `Hash + Eq + Clone + 'static` |
| `Value` bound | `Clone + Send + 'static` | `Clone + 'static` — so `Rc<T>` values work |
| Per-operation cost | atomic refcounts, `RwLock` guard | non-atomic refcounts, `RefCell` borrow |

**Start with `shared`.** It behaves like the old crate-root type and works under either
runtime flavor.

**The rule for `local` is about `spawn`, not about the runtime.** Its handles are not
`Send`, so nothing holding one can cross a `tokio::spawn` boundary — that is a compile
error, which is the point. A `current_thread` runtime is the natural fit, but `local`
is equally fine under a multi-threaded runtime as long as you stay inside
`block_on`/`spawn_local` and never `spawn`. Concurrency is still available there via
`spawn_local` on a [`LocalSet`] or by polling many futures on one task with
`futures::future::join_all`; what you give up is *parallelism*.

**What `local` buys** is non-atomic refcounting, no locking, and the ability to carry
non-`Send` values — `V = Rc<T>` compiles on `local` and cannot on `shared`.

Neither backend spawns a background task, so nothing here has to be *constructed*
inside a runtime, and `send` / `subscribe` / `n_keys` / `key_capacity` are plain
non-`async` functions you can call from anywhere. Only `recv` is a future, and since
it comes from `tokio::sync` any executor can poll it. `try_recv` is synchronous;
`blocking_recv` is for synchronous callers driving the channel from outside a runtime.

### Tokio feature flags

key-stream depends on tokio with **`sync` only**. It never spawns, so it does not pull
tokio's runtime into your build. You bring your own runtime and enable whatever it
needs — `rt`, `rt-multi-thread`, `macros` — and because Cargo unifies features
additively, nothing here constrains that choice.

Both backends expose the same API, so switching is a change of `use` path plus
whatever the `Send` bounds force.

[`LocalSet`]: https://docs.rs/tokio/latest/tokio/task/struct.LocalSet.html

## Usage

### Simple Usage

```rust
use key_stream::shared::KeyStream;

let key_stream = KeyStream::<String, String>::new(10);
let sender = key_stream.sender();
let mut receiver = sender.subscribe("1".to_string());
sender.send(&"1".to_string(), "value".to_string());
assert_eq!(receiver.recv().await.expect("recv failed"), "value".to_string());
```

### Receiver Dropping

If all receivers are dropped, the related key is also removed.
Messages sent to it will be ignored.
Key cleanup is synchronous on the last receiver drop.

```rust
use key_stream::shared::KeyStream;

let key_stream = KeyStream::<String, String>::new(10);
let sender = key_stream.sender();
let receiver = sender.subscribe("1".to_string());
assert_eq!(key_stream.n_keys(), 1);
drop(receiver);
let result = sender.send(&"1".to_string(), "value".to_string());
assert_eq!(result, 0);
assert_eq!(key_stream.n_keys(), 0);
```
