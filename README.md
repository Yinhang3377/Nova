# Nova

This repository contains the Nova blockchain scaffold. The full documentation site is published via GitHub Pages and the source lives under `docs/`.

Quick links:

- Docs site: https://Yinhang3377.github.io/Nova/  (after Pages is configured)
- Local docs: `mkdocs serve` (see `docs/`)

<!-- Docs badge placeholder (becomes active after Pages is enabled) -->
<!-- [![Docs](https://github.com/Yinhang3377/Nova/actions/workflows/docs.yml/badge.svg)](https://yinhang3377.github.io/Nova/) -->

For developer and API reference see the documentation pages under `docs/` (Contributing, Storage, Architecture).

<!-- Pages badge -->
[![Docs build status](https://github.com/Yinhang3377/Nova/actions/workflows/publish-docs.yml/badge.svg)](https://github.com/Yinhang3377/Nova/actions/workflows/publish-docs.yml)
[![Docs site](https://img.shields.io/website-up-down-green-red/https/yinhang3377.github.io/Nova/)](https://yinhang3377.github.io/Nova/)
# Nova
A new beginning

[![CI](https://github.com/Yinhang3377/Nova/actions/workflows/ci.yml/badge.svg)](https://github.com/Yinhang3377/Nova/actions/workflows/ci.yml)

[Release v0.1.0](https://github.com/Yinhang3377/Nova/releases/tag/v0.1.0)

## Nova (monorepo) - initial scaffold

This repository is a scaffold for the Nova blockchain project. It contains:

- `nova-core`: Core primitives and runtime components
- `nova-cli`: Command-line interface for wallets and node control
- `docs`: Design documents and roadmap
- `test-utils`: Test harness and simulators

This scaffold is intentionally minimal and designed to be expanded per the Nova project outline.

See developer notes in [docs/STORAGE.md](docs/STORAGE.md) for details on storage backends and the test-friendly factory.

## `nova-cli simulate` backend option

The `simulate` command supports a `--backend` flag to control persistence of generated blocks:

- `--backend mem` (default): store generated blocks in an in-memory storage backend (useful for tests and local simulations).
- `--backend none`: do not persist generated blocks (backward-compatible output-only behavior).

Examples:

```bash
# Run 5 simulated blocks and persist them in memory (default)
nova-cli simulate --count 5 --json

# Run 2 simulated blocks without persisting
nova-cli simulate --count 2 --json --backend none
```
