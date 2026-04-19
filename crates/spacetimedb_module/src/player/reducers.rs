use shared::{DisplayName, MovementIntent, Vec2f, simulate_movement};
use spacetimedb::{ReducerContext, Table};

use crate::player::tables::Player;
#[allow(unused_imports)]
use crate::player::tables::player as _player_table_trait;

#[spacetimedb::reducer]
pub fn connect_guest(ctx: &ReducerContext, guest_name: String) -> Result<(), String> {
    let name = DisplayName::new(&guest_name).map_err(|err| err.to_string())?;
    register_or_refresh(ctx, name);
    Ok(())
}

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

    let intent = MovementIntent::new(
        Vec2f {
            x: direction_x,
            y: direction_y,
        },
        delta_seconds,
    );
    let next_position = simulate_movement(Vec2f { x: row.x, y: row.y }, intent);
    row.x = next_position.x;
    row.y = next_position.y;
    ctx.db.player().id().update(row);
    Ok(())
}

pub fn register_or_refresh(ctx: &ReducerContext, name: DisplayName) {
    let Some(connection_id) = ctx.connection_id() else {
        return;
    };

    let name = name.into_inner();

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
