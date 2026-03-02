# Runbook

## Prerequisites

- Rust stable toolchain
- SpacetimeDB CLI/runtime available on path

## Development flow

1. Start SpacetimeDB runtime.
2. Build and publish module from `crates/spacetimedb_module`.
3. Run Bevy client from `crates/client_bevy`.
4. Start a second client instance to validate movement sync.

## Quality checks

- `cargo check --workspace`
- `cargo fmt --all`
- `cargo clippy --workspace --all-targets`
