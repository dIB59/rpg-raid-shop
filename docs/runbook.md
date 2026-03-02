# Runbook

## Prerequisites

- Rust stable toolchain
- SpacetimeDB CLI/runtime available on path (`spacetime`)

## One-time shell setup

Add SpacetimeDB CLI to your `PATH` in shell config:

`export PATH="$HOME/.local/bin:$PATH"`

## Start authoritative server

From the project root:

1. Start local SpacetimeDB node (host machine):
	- `spacetime start --listen-addr 0.0.0.0:3000 --in-memory --non-interactive`
2. Publish module:
	- `spacetime publish rpg-raid-shop -s http://127.0.0.1:3000 --anonymous -y --module-path crates/spacetimedb_module`
3. Generate/update Rust client bindings:
	- `spacetime generate --lang rust --out-dir crates/client_bevy/src/module_bindings --bin-path target/wasm32-unknown-unknown/release/spacetimedb_module.wasm -y`

## Development flow

1. On desktop A (or server host), run the authoritative server steps above.
2. On each desktop client, point to the same server + DB:
	- `SPACETIME_URI=http://<SERVER_IP>:3000 SPACETIME_DB=rpg-raid-shop SPACETIME_GUEST=Guest_A cargo run -p client_bevy`
	- `SPACETIME_URI=http://<SERVER_IP>:3000 SPACETIME_DB=rpg-raid-shop SPACETIME_GUEST=Guest_B cargo run -p client_bevy`
3. Move each client with `WASD`; positions should update on both desktops via SpacetimeDB.

## Quality checks

- `cargo check --workspace`
- `cargo fmt --all`
- `cargo clippy --workspace --all-targets`
