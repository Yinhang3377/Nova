# Nova v0.1.0 - Release Notes

This is an initial milestone release for the Nova monorepo.

Highlights
- Added minimal Proof-of-History (PoH) generator and unit tests.
- Implemented `nova-cli simulate` command with JSON output and integration tests.
- Restored placeholder modules so `nova-core` builds cleanly.
- CI workflow configuration: formatting, clippy, and test runs.
- Code passes `cargo clippy --all -- -D warnings` and basic test suite.

How to run
```bash
cargo test --workspace
cargo run -p nova-cli -- simulate --count 3 --json
```

Notes
- This is a milestone tag for internal development. Expect module implementations (VM, storage details) to be expanded in subsequent releases.
