#+ Contributing to Nova

Thanks for wanting to contribute! This document explains the project's workflow, CI checks, and how to run tests and linters locally so your PRs land cleanly.

## Quick checklist for PRs

- Fork the repository and create a feature branch.
- Run the formatting and lint checks locally:

```powershell
cargo fmt --all
cargo clippy --all -- -D warnings
```

- Run the test suite for the crate you changed. Examples:

```powershell
# run all workspace tests (slower)
cargo test --workspace

# run only nova-core tests
cargo test -p nova-core

# run only nova-cli tests
cargo test -p nova-cli
```

- If you modify public APIs or add persisted types, update `docs/ARCHITECTURE.md` and add unit tests under the owning crate.

- Make sure commits are atomic and follow conventional commit style where appropriate (e.g., `feat:`, `fix:`, `chore:`).

## CI checks

The repository uses GitHub Actions to enforce:

- `cargo fmt --all -- --check` (formatting)
- `cargo clippy --all -- -D warnings` (lints as errors)
- `cargo test --workspace --no-fail-fast` (unit and integration tests)
# A runtime smoke check that executes `nova-cli simulate --count 1 --json`

## Local development tips

- Use the repository's `rust-toolchain.toml` so your local toolchain matches CI:

```powershell
rustup override set stable
```

- To speed up iteration while developing a crate, run `cargo test -p <crate>` and use `cargo watch` (install via `cargo install cargo-watch`) for auto-running tests during editing.

- Run integration tests (the ones under `tests/`) with:

```powershell
cargo test -p nova-cli --test integration_simulate
```

## Reporting issues and feature requests

Open issues in the repository and use labels like `bug`, `enhancement`, or `question`. Provide a minimal reproduction when possible.

## License & Code of Conduct

By contributing you agree to license your contributions under the project's license (see `LICENSE`). Please follow the repository's code of conduct and be respectful.
