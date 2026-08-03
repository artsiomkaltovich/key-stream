# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `key_stream::local`, a **new** single-threaded backend built on `Rc<RefCell<_>>` with
  `Rc<broadcast::Sender<_>>` handles. It accepts non-`Send` values — `V = Rc<T>` now
  works, which was impossible before — and pays no atomic refcounting. It is
  deliberately not `Send`, so the compiler rejects using it across threads.
- Documented `send`'s delivery semantics on both backends. The underlying broadcast
  channel is **overwriting**: `send` never blocks and never reports "full". Writing
  to a full ring overwrites the oldest slot, and the receiver that had not read it
  observes `RecvError::Lagged(n)` on its next `recv`. There is no backpressure.

### Changed

- **Breaking:** `KeyStream`, `KeySender`, `KeyReceiver`, `Key` and `Value` are no
  longer exported from the crate root. The previous implementation lives on as
  **`key_stream::shared`**, reimplemented; `key_stream::local` is new alongside it.
  `key_stream::KeyStream` deliberately does not resolve, so neither backend is a
  silent default.

  **Migration: `use key_stream::KeyStream` → `use key_stream::shared::KeyStream`.**
  That is the like-for-like replacement — the old root type was
  `Arc<RwLock<_>>`-backed and usable across threads. Do **not** reach for `local`
  unless the code is genuinely confined to one thread: `local::KeyStream` is not
  `Send`, so switching to it will fail to compile in any `tokio::spawn` or
  `JoinSet` that the old type accepted.
- **Breaking:** `shared` uses `std::sync::RwLock` instead of `tokio::sync::RwLock`.
  No critical section ever spanned an `.await`, so the async lock added overhead
  without buying anything — and removing it is what allows `send` and `subscribe` to
  be plain `fn`. Lock poisoning is recovered from rather than propagated: a panic
  while holding the map lock can only land between `HashMap` operations, which cannot
  leave the map torn.
- `shared` stores `Arc<broadcast::Sender<V>>` as the map value rather than
  `broadcast::Sender<V>` directly. `send` must clone the handle out to release the
  guard before broadcasting, and cloning an `Arc` costs one relaxed refcount bump,
  whereas cloning a `broadcast::Sender` also bumps tokio's separate `num_tx` counter
  whose `Drop` decrement is unconditionally `AcqRel` — measured at +5.8 ns (arm64) to
  +9.3 ns (x86_64) per send.
- **Breaking:** `Key` and `Value` are now defined per backend with different bounds.
  `local::Key` is `Hash + Eq + Clone + 'static` and `local::Value` is
  `Clone + 'static`; `shared::Key` additionally requires `Send + Sync` and
  `shared::Value` additionally requires `Send` (but **not** `Sync`).
- **Breaking:** `KeySender::send`, `KeySender::subscribe`, and `n_keys` /
  `key_capacity` on both `KeyStream` and `KeySender` are now plain `fn` instead of
  `async fn` — none of them ever awaited on either backend. **Migration:** drop the
  `.await`.
- **Breaking:** `KeySender::send` now returns `usize` instead of
  `Result<usize, SendError<V>>`.
- **Breaking:** `KeyReceiver::try_recv` is now synchronous (`fn`) instead of
  `async fn`.
- **Breaking:** Renamed `KeyReceiver::to_async_stream` to
  `KeyReceiver::into_stream`.
- **Breaking:** Key reclamation moved from a background cleanup task to
  synchronous cleanup in `KeyReceiver::drop`. Keys are now removed
  immediately when the last receiver drops; `KeyStream::new` no longer calls
  `tokio::spawn` and so no longer has to be constructed inside a Tokio runtime;
  and `KeyStream` no longer implements `Drop`.
- Updated `KeyStream::new` panic docs: it now only panics when
  `broadcast_capacity == 0`.
- Dropped the `rt` feature from the tokio dependency. Nothing in the library spawns
  any more, so the runtime is no longer pulled into downstream builds; only `sync` is
  required. Tests and benches keep it via dev-dependencies.
- Library code contains no `unwrap` or `expect`, enforced by
  `clippy::unwrap_used = "deny"` and `clippy::expect_used = "deny"`. Two documented
  panics remain: `KeyStream::new` on a zero capacity, and `KeyReceiver::blocking_recv`
  if called from within an async context — the latter inherited from
  `tokio::sync::broadcast::Receiver::blocking_recv` and now documented on our method
  too.
- Replaced archived `actions-rs/toolchain` with
  `dtolnay/rust-toolchain@stable` in CI and publish workflows.

### Fixed

- `KeySender::send` no longer reports an error when the key is present but all of its
  receivers have already dropped; it now returns `0`, matching the missing-key case
  and the documented behavior.
- The stream returned by `KeyReceiver::into_stream` now terminates when the
  channel is closed, instead of yielding `Err(RecvError::Closed)` forever.
- `KeySender::send` now releases the key-map guard before calling
  `broadcast::Sender::send`, which previously held it across the broadcast. Sending
  overwrites a ring slot and drops the evicted value, so a value destructor that
  touches the same stream would re-enter the guard — panicking on `local`'s `RefCell`
  and deadlocking `shared`'s `RwLock`.

## [0.10.0]

### Changed

- **Breaking:** Renamed `KeyStream::keys_capacity` to `KeyStream::key_capacity` for consistency with `KeySender::key_capacity`.

## [0.9.2]

### Changed

- Add readme field to `cargo.toml`

## [0.9.1]

### Changed

- Renamed `readme.md` to `README.md` to make the file discoverable by crates.io.
- Enabled Trusted Publishing.

## [0.9.0]

### Added
- Initial public release of async key-based message streaming library.
- Key-based message routing with automatic cleanup of unused keys.
- `KeyStream`, `KeySender`, and `KeyReceiver` types.
- Async and blocking message receive APIs.
- Automatic memory optimization for key map.
- Comprehensive tests for concurrency, lag, drop, and memory.
- CI with build, lint, format, test, and doc steps.
- GitHub Actions workflow for publishing to crates.io.
- Full API documentation and usage examples.
