# Runbook

## Prerequisites

- Rust stable toolchain
- SpacetimeDB CLI/runtime available on path (`spacetime`)

## One-time shell setup

Add SpacetimeDB CLI to your `PATH` in shell config:

`export PATH="$HOME/.local/bin:$PATH"`

Optional: copy `.env.dev.example` to `.env.dev` and customize values:

`cp .env.dev.example .env.dev`

## Start authoritative server

From the project root, use the lifecycle commands:

1. Bring up local DB + publish module + generate bindings:
   - `cargo dev-up`

By default, local DB data is persistent at `target/dev/spacetime-data`.
Override with `SPACETIME_DATA_DIR` if needed.

When finished, stop the managed local DB process:

- `cargo dev-down`

Run a client with an optional guest name:

- `cargo dev-client`
- `cargo dev-client Guest_A`
- `cargo dev-client Guest_B`

If needed, you can run pieces separately with `cargo db-publish` and `cargo db-generate`.

Tip: inspect effective DB config with:

`cargo db-config`

If you want a clean local DB state, stop it and remove the data directory:

1. `cargo dev-down`
2. `rm -rf target/dev/spacetime-data`

## Development flow

1. On desktop A (or server host), run the authoritative server steps above.
2. On each desktop client, point to the same server + DB:
   - `SPACETIME_URI=http://<SERVER_IP>:3000 SPACETIME_DB=rpg-raid-shop-dev SPACETIME_GUEST=Guest_A cargo run -p client_bevy`
   - `SPACETIME_URI=http://<SERVER_IP>:3000 SPACETIME_DB=rpg-raid-shop-dev SPACETIME_GUEST=Guest_B cargo run -p client_bevy`
3. Move each client with `WASD`; positions should update on both desktops via SpacetimeDB.

### Local single-machine quick test

1. Terminal 1: `cargo dev-up`
2. Terminal 2: `cargo dev-client Guest_A`
3. Terminal 3: `cargo dev-client Guest_B`

## Quality checks

- `cargo check --workspace`
- `cargo fmt --all`
- `cargo clippy --workspace --all-targets`
