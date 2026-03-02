use bevy::prelude::*;
use spacetimedb_sdk::{DbContext, Table};
use std::env;

use crate::module_bindings::{
    self, PlayerTableAccess, connect_guest_reducer::connect_guest, move_self_reducer::move_self,
};
use shared::{MovementIntent, PlayerId, PlayerState, Vec2f};

const DEFAULT_SPACETIME_URI: &str = "http://127.0.0.1:3000";
const DEFAULT_SPACETIME_DB: &str = "rpg-raid-shop-dev";

#[derive(Resource, Default)]
pub struct NetworkSnapshot {
    pub local_player: Option<PlayerState>,
    pub remote_players: Vec<PlayerState>,
}

#[derive(Resource)]
struct LiveConnection {
    connection: module_bindings::DbConnection,
    _subscription: module_bindings::SubscriptionHandle,
}

#[derive(Resource)]
struct LocalConnectionId {
    connection_id: spacetimedb_sdk::ConnectionId,
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkSnapshot>()
            .add_systems(Startup, bootstrap_live_connection)
            .add_systems(
                Update,
                (
                    send_local_intent_to_server,
                    poll_local_connection_id,
                    pull_snapshot_from_server,
                ),
            );
    }
}

fn bootstrap_live_connection(mut commands: Commands, mut snapshot: ResMut<NetworkSnapshot>) {
    let uri = env::var("SPACETIME_URI").unwrap_or_else(|_| DEFAULT_SPACETIME_URI.to_string());
    let database_name =
        env::var("SPACETIME_DB").unwrap_or_else(|_| DEFAULT_SPACETIME_DB.to_string());
    let guest_name = env::var("SPACETIME_GUEST").unwrap_or_else(|_| {
        let process_id = std::process::id();
        format!("Guest_{process_id}")
    });

    info!("Connecting client: uri={uri}, db={database_name}, guest={guest_name}");

    let connection = module_bindings::DbConnection::builder()
        .with_uri(&uri)
        .with_database_name(&database_name)
        .build()
        .unwrap_or_else(|error| {
            panic!(
                "failed to connect to SpacetimeDB (uri={uri}, db={database_name}). error={error}. \
Hint: run `cargo db-start` and `cargo db-sync`, then relaunch the client."
            )
        });

    let subscription = connection
        .subscription_builder()
        .subscribe("SELECT * FROM player");
    connection
        .reducers
        .connect_guest(guest_name.clone())
        .unwrap_or_else(|error| {
            panic!("failed to call connect_guest reducer for guest={guest_name}. error={error}")
        });

    connection.run_threaded();

    commands.insert_resource(LiveConnection {
        connection,
        _subscription: subscription,
    });

    snapshot.local_player = None;
    snapshot.remote_players.clear();
}

fn send_local_intent_to_server(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    live: Option<Res<LiveConnection>>,
) {
    let Some(live) = live else {
        return;
    };

    let local_direction_x = axis(&keys, KeyCode::KeyA, KeyCode::KeyD);
    let local_direction_y = axis(&keys, KeyCode::KeyS, KeyCode::KeyW);

    let local_intent = MovementIntent {
        direction: Vec2f {
            x: local_direction_x,
            y: local_direction_y,
        },
        delta_seconds: time.delta_secs(),
    };
    if local_intent.direction.length_squared() <= f32::EPSILON {
        return;
    }

    let _ = live.connection.reducers.move_self(
        local_intent.direction.x,
        local_intent.direction.y,
        local_intent.delta_seconds,
    );
}

fn poll_local_connection_id(
    mut commands: Commands,
    live: Option<Res<LiveConnection>>,
    existing: Option<Res<LocalConnectionId>>,
) {
    if existing.is_some() {
        return;
    }

    let Some(live) = live else {
        return;
    };

    if let Some(connection_id) = live.connection.try_connection_id() {
        commands.insert_resource(LocalConnectionId { connection_id });
    }
}

fn pull_snapshot_from_server(
    live: Option<Res<LiveConnection>>,
    local_connection_id: Option<Res<LocalConnectionId>>,
    mut snapshot: ResMut<NetworkSnapshot>,
) {
    let Some(live) = live else {
        return;
    };

    let all_players: Vec<PlayerState> = live
        .connection
        .db
        .player()
        .iter()
        .map(|player| PlayerState {
            id: PlayerId(player.id),
            name: player.name,
            position: Vec2f {
                x: player.x,
                y: player.y,
            },
        })
        .collect();

    let local_player_id = local_connection_id.and_then(|local_connection_id| {
        live.connection
            .db
            .player()
            .connection_id()
            .find(&local_connection_id.connection_id)
            .map(|player| PlayerId(player.id))
    });

    snapshot.local_player = all_players
        .iter()
        .find(|player| Some(player.id) == local_player_id)
        .cloned();

    snapshot.remote_players = all_players
        .into_iter()
        .filter(|player| Some(player.id) != local_player_id)
        .collect();
}

fn axis(keys: &ButtonInput<KeyCode>, negative: KeyCode, positive: KeyCode) -> f32 {
    let mut value = 0.0;
    if keys.pressed(negative) {
        value -= 1.0;
    }
    if keys.pressed(positive) {
        value += 1.0;
    }
    value
}
