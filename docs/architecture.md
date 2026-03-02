# Architecture basics

## Principles

- SpacetimeDB is authoritative for mutable multiplayer state.
- Client predicts local movement and reconciles from authoritative snapshots.
- LDTK is the source of truth for static map content (layers, collisions, spawn points).

## Crate boundaries

- `shared`: IDs, vectors, movement intents, and player state DTOs.
- `spacetimedb_module`: reducer-side simulation and persistence model.
- `client_bevy`: rendering, input capture, prediction/interpolation, network bridge.

## Data flow (MVP)

1. Client starts with guest identity.
2. Client sends movement intent each frame/tick.
3. Server applies speed/step limits and writes authoritative position.
4. Server broadcasts updates.
5. Client reconciles local entity and interpolates remote entities.
