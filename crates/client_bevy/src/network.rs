use bevy::prelude::*;
use shared::{MovementIntent, PlayerId, PlayerState, Vec2f};

#[derive(Resource, Default)]
pub struct NetworkSnapshot {
    pub local_player: Option<PlayerState>,
    pub remote_players: Vec<PlayerState>,
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkSnapshot>()
            .add_systems(Startup, bootstrap_guest_identity)
            .add_systems(Update, simulate_authoritative_echo);
    }
}

fn bootstrap_guest_identity(mut snapshot: ResMut<NetworkSnapshot>) {
    snapshot.local_player = Some(PlayerState {
        id: PlayerId(1),
        name: "Guest_1".to_string(),
        position: Vec2f::default(),
    });

    snapshot.remote_players = vec![PlayerState {
        id: PlayerId(2),
        name: "Guest_2".to_string(),
        position: Vec2f { x: 128.0, y: 64.0 },
    }];
}

fn simulate_authoritative_echo(time: Res<Time>, mut snapshot: ResMut<NetworkSnapshot>) {
    let speed = 120.0;
    let intent = MovementIntent {
        direction: Vec2f { x: 1.0, y: 0.0 },
        delta_seconds: time.delta_secs(),
    };

    if let Some(remote) = snapshot.remote_players.first_mut() {
        let delta = intent
            .direction
            .normalize_or_zero()
            .scaled(speed * intent.delta_seconds);
        remote.position.x += delta.x;
        if remote.position.x > 240.0 {
            remote.position.x = -240.0;
        }
    }
}
