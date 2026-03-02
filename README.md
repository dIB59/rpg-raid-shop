# rpg-raid-shop

Multiplayer 2D top-down RPG foundation using Rust, Bevy, LDTK, and SpacetimeDB.

## Workspace

- `crates/client_bevy`: desktop Bevy client
- `crates/shared`: protocol + domain types shared by client systems
- `crates/spacetimedb_module`: SpacetimeDB tables, reducers, and public views

## Current milestone

- SpacetimeDB authoritative `player` table + reducers (`connect_guest`, `move_self`)
- Generated Rust client bindings in `crates/client_bevy/src/module_bindings`
- Bevy client connected to live SpacetimeDB (table subscription + reducer calls)
- Local and remote player squares synced from authoritative table state

## Next implementation targets

1. Add LDTK world loading and collision extraction.
2. Add client-side interpolation/smoothing for remote players.
3. Add combat reducers and hit resolution.
