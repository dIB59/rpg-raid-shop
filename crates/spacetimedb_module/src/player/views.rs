use spacetimedb::{Query, ViewContext};

use crate::player::tables::Player;
#[allow(unused_imports)]
use crate::player::tables::player__query as _player_query_trait;

#[spacetimedb::view(accessor = players_snapshot, public)]
pub fn players_snapshot(ctx: &ViewContext) -> impl Query<Player> {
    ctx.from.player().build()
}
