# Plumb

Plumb is an open-source, Rust-first 3D modeling environment built around a shared scene engine.
The engine exposes explicit operations that desktop tools, command-line workflows, scripts, and agent interfaces can share.

![Architecture diagram showing the implemented CLI and planned desktop and agent interfaces sharing plumb-core, which serializes scene data as JSON.](docs/architecture.svg)

## What works today

The `plumb-core` crate provides versioned scene data, stable object IDs, box creation, position changes, validation, and JSON serialization.
The `plumb-cli` application creates and edits scene files and prints their contents as JSON.

The desktop application, viewport, and agent protocol are planned and are not implemented yet.

## Build

Install a stable Rust toolchain, then run:

```sh
cargo build --workspace
cargo test --workspace
```

## CLI example

```sh
cargo run -p plumb-cli -- new scene.json
cargo run -p plumb-cli -- add-box scene.json block 2 3 4
cargo run -p plumb-cli -- move scene.json 1 1 0 0
cargo run -p plumb-cli -- inspect scene.json
```

Scene dimensions and positions use scene units.
CLI diagnostics are written to standard error, while inspection output is JSON on standard output.

## Workspace

- `crates/plumb-core/` contains scene types, validation, modeling operations, and serialization.
- `apps/plumb-cli/` exposes the core operations in a scriptable command-line interface.

## Development

Run `cargo fmt --all`, `cargo check --workspace`, and `cargo test --workspace` before proposing changes.
Keep scene semantics in `plumb-core` so additional interfaces can reuse the same behavior.
