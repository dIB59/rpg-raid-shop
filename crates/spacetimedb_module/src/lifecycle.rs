//! Connection lifecycle reducers. Currently dispatches straight into `player`;
//! once accounts land this will resolve `identity -> account -> character` instead.

use shared::DisplayName;
use spacetimedb::{Identity, ReducerContext};

use crate::player;
#[allow(unused_imports)]
use crate::player::tables::player as _player_table_trait;

#[spacetimedb::reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
    player::register_or_refresh(ctx, default_guest_name(ctx.sender()));
}

#[spacetimedb::reducer(client_disconnected)]
pub fn client_disconnected(ctx: &ReducerContext) {
    if let Some(connection_id) = ctx.connection_id() {
        ctx.db.player().connection_id().delete(connection_id);
    }
}

fn default_guest_name(identity: Identity) -> DisplayName {
    let identity_text = identity.to_string();
    let suffix: String = identity_text.chars().take(8).collect();
    DisplayName::new(&format!("Guest_{suffix}"))
        .expect("identity-derived guest names are always valid")
}
