## Quick orientation for AI contributors

This monorepo contains a minimal scaffold for the Nova blockchain. The goal of
these instructions is to help an AI coding agent be immediately productive by
calling out the project's architecture, build/test workflows, and code patterns
that appear across the repository.

Core layout
- `nova-core/` — core primitives and runtime modules (see `nova-core/src/*.rs`).
  Important modules: `data_model.rs`, `consensus.rs`, `vm.rs`, `network.rs`,
  `storage.rs`, `bridge.rs`, `wallet.rs`, `governance.rs`.
- `nova-cli/` — small CLI built with `clap` (`nova-cli/src/main.rs`) used to
  exercise wallet, tx, gov, and simulate commands.
- `docs/ARCHITECTURE.md` — high-level design notes; treat as the canonical
  project intent when reasoning about module responsibilities.

Big picture and data flow
- `nova-core::data_model` defines Block/Transaction shapes (see
  `nova-core/src/data_model.rs`) — most modules operate on these types.
- Consensus (PoH, BFT quorum) is stenciled under `nova-core::consensus` and is
  expected to produce block headers (`poh_digest`, `number`, `timestamp`) that
  feed into storage and networking layers.
- The VM layer (`vm.rs`) is a thin abstraction for execution (WASM/EVM
  compatibility planned). Treat the VM as a pure function: state + tx -> new
  state + receipts.

Build, test, and CI workflows
- This is a Rust workspace. Use `cargo build --workspace`,
  `cargo test --workspace --no-fail-fast`, and `cargo fmt --all` locally to
  match CI which runs these steps (see `.github/workflows/ci.yml`).
- CI enforces `cargo clippy --all -D warnings`. New code must be linter-clean.
- For iterative development prefer `cargo test -p <crate>` to focus on a crate.

Project-specific conventions
- Keep public API surface minimal in `nova-core` modules — the repo currently
  uses skeletal submodules (e.g., `poh`, `driftrd`, `bft_quorum`) and expects
  implementations to live under those module names.
- Serialization uses `serde` for Block/Transaction structs in
  `data_model.rs` — use `serde::{Serialize, Deserialize}` when adding persisted
  types.
- CLI commands are defined with `clap` in `nova-cli/src/main.rs`. Sample
  patterns: `Commands::Wallet { action }`, `Commands::Tx { to, amount }`.

Integration points and cross-component patterns
- Crates communicate via the workspace; prefer internal crate paths (e.g.
  `nova_core::data_model::Block`) rather than duplicating types.
- Storage and networking modules are thin stubs. When adding integration tests
  place them under `test-utils/` and exercise the CLI for higher-level flows.

What to change and how to verify
- When adding new public interfaces: update docs in `docs/ARCHITECTURE.md` and
  add unit tests in the owning crate. Run `cargo test --workspace` to verify.
- When adding features touching consensus or VM, include small simulation
  examples in `nova-cli` (`simulate` command) to exercise the change end-to-end.

Examples (searchable pointers)
- Data types: `nova-core/src/data_model.rs`
- CLI entrypoint: `nova-cli/src/main.rs`
- CI: `.github/workflows/ci.yml`
- Workspace config: `Cargo.toml` (top-level)

If something is ambiguous
- Prefer conservative changes that keep existing module boundaries. If you need
  to add or rename crates, update the top-level `Cargo.toml` and CI workflow.
- If runtime secrets, external networks, or infra are required, flag the change
  and ask maintainers; don't hardcode keys or endpoints.

Questions for maintainers
- Is there an intended target Rust toolchain (stable/nightly)? The repo
  contains `rust-toolchain.toml` but ensure you pick the toolchain used in CI.
- Which modules are currently considered stable vs experimental (helpful for
  prioritizing changes)?

Please review these instructions and tell me what else you'd like the agent to
know (CI tokens, testing harnesses, or example datasets to run simulations).
