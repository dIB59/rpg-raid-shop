# rpg-raid-shop

Multiplayer 2D top-down RPG foundation using Rust, Bevy, LDTK, and SpacetimeDB.

## Workspace

- `crates/client_bevy`: desktop Bevy client
- `crates/shared`: protocol + domain types shared by client systems
- `crates/spacetimedb_module`: SpacetimeDB tables, reducers, and public views

## Current milestone

- SpacetimeDB `player` table with lifecycle reducers (`client_connected`, `client_disconnected`)
- Guest registration reducer (`connect_guest`) and authoritative movement reducer (`move_self`)
- Public snapshot view (`players_snapshot`) for multiplayer sync
- Bevy client network layer aligned to module API contract (`connect_guest`, `move_self`, snapshot pull)

## Next implementation targets

1. Generate Rust client bindings from the module and replace the temporary in-process authoritative adapter.
2. Add LDTK world loading and collision extraction.
3. Add remote-player interpolation + reconciliation against subscribed snapshots.
