use spacetimedb::{ConnectionId, Identity};

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
