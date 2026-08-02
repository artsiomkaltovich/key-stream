# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **Breaking:** `KeySender::send` now returns `usize` instead of
  `Result<usize, SendError<V>>`.
- **Breaking:** `KeyReceiver::try_recv` is now synchronous (`fn`) instead of
  `async fn`.
- **Breaking:** Renamed `KeyReceiver::to_async_stream` to
  `KeyReceiver::into_stream`.
- **Breaking:** Key reclamation moved from a background cleanup task to
  synchronous cleanup in `KeyReceiver::drop`. Keys are now removed
  immediately when the last receiver drops; `KeyStream::new` no longer
  spawns a task and no longer requires running inside a Tokio runtime or
  `LocalSet`; and `KeyStream` no longer implements `Drop`.
- Updated `KeyStream::new` panic docs: it now only panics when
  `broadcast_capacity == 0`.
- Replaced archived `actions-rs/toolchain` with
  `dtolnay/rust-toolchain@stable` in CI and publish workflows.

### Fixed

- `KeySender::send` no longer reports an error when the key's receivers were all
  dropped but the cleanup task has not removed the key yet; it now returns `0`,
  matching the documented behavior and the missing-key case.
- The stream returned by `KeyReceiver::into_stream` now terminates when the
  channel is closed, instead of yielding `Err(RecvError::Closed)` forever.
- `KeySender::send` now releases the key map borrow before `broadcast::Sender::send`,
  preventing reentrant-drop borrow panics in local mode and lock reentry hazards
  (including deadlock risk on lock-based backends) when value destructors touch the
  stream.

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
