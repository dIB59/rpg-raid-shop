//! SpacetimeDB module that owns authoritative player state and movement reducers.

use spacetimedb::{ConnectionId, Identity, Query, ReducerContext, Table, ViewContext};

const PLAYER_SPEED_UNITS_PER_SEC: f32 = 180.0;
const MAX_SIMULATION_STEP_SECONDS: f32 = 0.1;

/// Persistent player record stored in SpacetimeDB.
#[spacetimedb::table(accessor = player, public)]
pub struct Player {
    #[auto_inc]
    #[primary_key]
    pub id: u64,
    pub identity: Identity,
    #[unique]
    pub connection_id: ConnectionId,
    pub name: String,
    pub x: f32,
    pub y: f32,
}

/// Ensures a player row exists when a client connects.
#[spacetimedb::reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
    ensure_player_exists(ctx, default_guest_name(ctx.sender()));
}

/// Removes the player row for the disconnecting connection.
#[spacetimedb::reducer(client_disconnected)]
pub fn client_disconnected(ctx: &ReducerContext) {
    if let Some(connection_id) = ctx.connection_id() {
        ctx.db.player().connection_id().delete(connection_id);
    }
}

/// Registers or updates the caller as a guest with a sanitized display name.
#[spacetimedb::reducer]
pub fn connect_guest(ctx: &ReducerContext, guest_name: String) -> Result<(), String> {
    let sanitized_name = sanitize_guest_name(guest_name)?;
    ensure_player_exists(ctx, sanitized_name);
    Ok(())
}

/// Applies normalized movement input for the caller over a clamped delta time.
#[spacetimedb::reducer]
pub fn move_self(
    ctx: &ReducerContext,
    direction_x: f32,
    direction_y: f32,
    delta_seconds: f32,
) -> Result<(), String> {
    let connection_id = ctx
        .connection_id()
        .ok_or_else(|| "missing connection id for caller".to_string())?;

    let mut row = ctx
        .db
        .player()
        .connection_id()
        .find(connection_id)
        .ok_or_else(|| "player not registered, call connect_guest first".to_string())?;

    let clamped_dt = delta_seconds.clamp(0.0, MAX_SIMULATION_STEP_SECONDS);
    if clamped_dt <= f32::EPSILON {
        return Ok(());
    }

    let (dir_x, dir_y) = normalize_or_zero(direction_x, direction_y);
    row.x += dir_x * PLAYER_SPEED_UNITS_PER_SEC * clamped_dt;
    row.y += dir_y * PLAYER_SPEED_UNITS_PER_SEC * clamped_dt;
    ctx.db.player().id().update(row);
    Ok(())
}

/// Returns the complete set of players for client-side snapshot rendering.
#[spacetimedb::view(accessor = players_snapshot, public)]
pub fn players_snapshot(ctx: &ViewContext) -> impl Query<Player> {
    ctx.from.player().build()
}

/// Ensures there is exactly one player row for the current connection.
///
/// If a row already exists, only mutable fields are refreshed.
fn ensure_player_exists(ctx: &ReducerContext, name: String) {
    let Some(connection_id) = ctx.connection_id() else {
        return;
    };

    if let Some(existing) = ctx.db.player().connection_id().find(connection_id) {
        ctx.db.player().id().update(Player {
            id: existing.id,
            identity: existing.identity,
            connection_id: existing.connection_id,
            name,
            x: existing.x,
            y: existing.y,
        });
        return;
    }

    ctx.db.player().insert(Player {
        id: 0,
        identity: ctx.sender(),
        connection_id,
        name,
        x: 0.0,
        y: 0.0,
    });
}

/// Validates and normalizes a guest name to alphanumeric, `_`, and `-` (max 24 chars).
fn sanitize_guest_name(candidate: String) -> Result<String, String> {
    let trimmed = candidate.trim();
    if trimmed.is_empty() {
        return Err("guest name cannot be empty".to_string());
    }

    let mut normalized = String::with_capacity(trimmed.len().min(24));
    for character in trimmed.chars() {
        if character.is_alphanumeric() || character == '_' || character == '-' {
            normalized.push(character);
        }

        if normalized.len() >= 24 {
            break;
        }
    }

    if normalized.is_empty() {
        return Err("guest name must contain letters or numbers".to_string());
    }

    Ok(normalized)
}

fn default_guest_name(identity: Identity) -> String {
    let identity_text = identity.to_string();
    let suffix: String = identity_text.chars().take(8).collect();
    format!("Guest_{suffix}")
}

/// Normalizes `(x, y)` or returns `(0, 0)` when magnitude is near zero.
fn normalize_or_zero(x: f32, y: f32) -> (f32, f32) {
    let len_sq = (x * x) + (y * y);
    if len_sq <= f32::EPSILON {
        return (0.0, 0.0);
    }

    let inv_len = len_sq.sqrt().recip();
    (x * inv_len, y * inv_len)
}
