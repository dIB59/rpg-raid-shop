use spacetimedb::{Identity, Query, ReducerContext, Table, ViewContext};

const PLAYER_SPEED_UNITS_PER_SEC: f32 = 180.0;
const MAX_SIMULATION_STEP_SECONDS: f32 = 0.1;

#[spacetimedb::table(accessor = player, public)]
pub struct Player {
    #[auto_inc]
    #[primary_key]
    pub id: u64,
    #[unique]
    pub identity: Identity,
    pub name: String,
    pub x: f32,
    pub y: f32,
}

#[spacetimedb::reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
    ensure_player_exists(ctx, default_guest_name(ctx.sender()));
}

#[spacetimedb::reducer(client_disconnected)]
pub fn client_disconnected(ctx: &ReducerContext) {
    ctx.db.player().identity().delete(ctx.sender());
}

#[spacetimedb::reducer]
pub fn connect_guest(ctx: &ReducerContext, guest_name: String) -> Result<(), String> {
    let sanitized_name = sanitize_guest_name(guest_name)?;
    ensure_player_exists(ctx, sanitized_name);
    Ok(())
}

#[spacetimedb::reducer]
pub fn move_self(
    ctx: &ReducerContext,
    direction_x: f32,
    direction_y: f32,
    delta_seconds: f32,
) -> Result<(), String> {
    let mut row = ctx
        .db
        .player()
        .identity()
        .find(ctx.sender())
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

#[spacetimedb::view(accessor = players_snapshot, public)]
pub fn players_snapshot(ctx: &ViewContext) -> impl Query<Player> {
    ctx.from.player().build()
}

fn ensure_player_exists(ctx: &ReducerContext, name: String) {
    if let Some(existing) = ctx.db.player().identity().find(ctx.sender()) {
        ctx.db.player().id().update(Player {
            id: existing.id,
            identity: existing.identity,
            name,
            x: existing.x,
            y: existing.y,
        });
        return;
    }

    ctx.db.player().insert(Player {
        id: 0,
        identity: ctx.sender(),
        name,
        x: 0.0,
        y: 0.0,
    });
}

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

fn normalize_or_zero(x: f32, y: f32) -> (f32, f32) {
    let len_sq = (x * x) + (y * y);
    if len_sq <= f32::EPSILON {
        return (0.0, 0.0);
    }

    let inv_len = len_sq.sqrt().recip();
    (x * inv_len, y * inv_len)
}
