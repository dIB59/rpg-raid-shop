# rpg-raid-shop

Multiplayer 2D top-down RPG foundation using Rust, Bevy, LDTK, and SpacetimeDB.

## Workspace

- `crates/client_bevy`: desktop Bevy client
- `crates/shared`: protocol + domain types shared by client/module
- `crates/spacetimedb_module`: server-authoritative simulation skeleton

## Current milestone

- Guest identity bootstrap
- Movement intent type definitions
- Authoritative movement simulation skeleton
- Bevy client plugin structure for networking + player sync

## Next implementation targets

1. Replace in-memory simulation with real SpacetimeDB tables/reducers.
2. Add LDTK world loading and collision extraction.
3. Add client subscription + interpolation for remote players.
