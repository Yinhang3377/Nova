# Storage backends and factory

This document explains the minimal `Storage` trait and the `factory::open_backend` helper used by the `nova-cli` simulation command.

## Storage trait

Located at `nova-core/src/storage.rs`. The trait is intentionally minimal for the scaffold:

- put(key: Vec<u8>, value: Vec<u8>)
- get(key: &[u8]) -> Option<Vec<u8>>
- delete(key: &[u8])
- contains_key(key: &[u8]) -> bool

Keys and values are raw byte vectors to keep the interface low-level and flexible for different backends.

## Factory: `open_backend(name)`

Implemented in `nova-core/src/storage/factory.rs`. It returns a boxed `Storage` trait object for a backend name. Current supported names:

- `mem` — in-memory `MemDb` (default)
- `rocks` — `RocksDbStub` placeholder (delegates to MemDb). Replace with a RocksDB adapter when ready.
- `sled` — `SledDbStub` placeholder (delegates to MemDb). Replace with a sled adapter when ready.

If an unknown name is provided, the factory currently falls back to `mem`.

## How to add a real backend

1. Add a module under `nova-core/src/storage/<backend>.rs`.
2. Implement the `Storage` trait for your backend type. Use `Send + Sync` semantics.
3. Wire the backend into `factory::open_backend` so it can be chosen by name.
4. Add tests that exercise put/get/delete and concurrency where appropriate.

If the backend requires native dependencies (e.g. RocksDB), add them to `nova-core/Cargo.toml` and update CI to install any required system packages.

## Testing

- The `nova-cli simulate` command uses `factory::open_backend` and the `--backend` flag to choose a backend. The integration test `nova-cli/tests/integration_simulate.rs` verifies the default `mem` behavior and `--backend none` behavior.

## Notes

These adapters are intentionally minimal. They provide a stable interface for upper layers to integrate real storage backends later.
