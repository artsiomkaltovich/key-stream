# Changelog

All notable changes to this project will be documented in this file.
c
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
