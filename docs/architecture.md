# Architecture basics

## Principles

- SpacetimeDB is authoritative for mutable multiplayer state.
- Client predicts local movement and reconciles from authoritative snapshots.
- LDTK is the source of truth for static map content (layers, collisions, spawn points).

## Crate boundaries

- `shared`: IDs, vectors, movement intents, and player state DTOs for client systems.
- `spacetimedb_module`: authoritative `player` table, reducers, and public read views.
- `client_bevy`: rendering, input capture, prediction/interpolation, and module API adapter.

## Data flow (MVP)

1. Client starts with guest identity.
2. Client sends movement intent each frame/tick.
3. `move_self` applies speed/step limits and writes authoritative position.
4. Clients query/subscribe to `players_snapshot`.
5. Client reconciles local entity and interpolates remote entities.
