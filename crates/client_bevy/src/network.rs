use bevy::prelude::*;
use shared::{MovementIntent, PlayerId, PlayerState, Vec2f};

#[derive(Resource, Default)]
pub struct NetworkSnapshot {
    pub local_player: Option<PlayerState>,
    pub remote_players: Vec<PlayerState>,
}

#[derive(Resource, Default)]
struct AuthoritativeApi {
    players: Vec<PlayerState>,
    local_player_id: Option<PlayerId>,
}

impl AuthoritativeApi {
    fn connect_guest(&mut self, guest_name: String) -> PlayerState {
        if let Some(local_player_id) = self.local_player_id
            && let Some(existing) = self
                .players
                .iter_mut()
                .find(|player| player.id == local_player_id)
        {
            existing.name = guest_name;
            return existing.clone();
        }

        let id = PlayerId((self.players.len() + 1) as u64);
        let state = PlayerState {
            id,
            name: guest_name,
            position: Vec2f::default(),
        };

        self.local_player_id = Some(id);
        self.players.push(state.clone());

        if self.players.len() == 1 {
            self.players.push(PlayerState {
                id: PlayerId(2),
                name: "Guest_Bot".to_string(),
                position: Vec2f { x: 128.0, y: 64.0 },
            });
        }

        state
    }

    fn move_self(&mut self, intent: MovementIntent) {
        let Some(local_player_id) = self.local_player_id else {
            return;
        };

        let speed = 180.0;
        let clamped_dt = intent.delta_seconds.clamp(0.0, 0.1);
        let direction = intent.direction.normalize_or_zero();
        let delta = direction.scaled(speed * clamped_dt);

        if let Some(local) = self
            .players
            .iter_mut()
            .find(|player| player.id == local_player_id)
        {
            local.position.x += delta.x;
            local.position.y += delta.y;
        }
    }

    fn tick_bots(&mut self, elapsed_seconds: f32) {
        if let Some(bot) = self
            .players
            .iter_mut()
            .find(|player| player.name == "Guest_Bot")
        {
            bot.position.x = elapsed_seconds.sin() * 180.0;
            bot.position.y = elapsed_seconds.cos() * 120.0;
        }
    }

    fn players_snapshot(&self) -> Vec<PlayerState> {
        self.players.clone()
    }
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkSnapshot>()
            .init_resource::<AuthoritativeApi>()
            .add_systems(Startup, bootstrap_guest_identity)
            .add_systems(
                Update,
                (
                    send_local_intent_to_authoritative,
                    tick_authoritative_world,
                    pull_authoritative_snapshot,
                ),
            );
    }
}

fn bootstrap_guest_identity(
    mut api: ResMut<AuthoritativeApi>,
    mut snapshot: ResMut<NetworkSnapshot>,
) {
    let local = api.connect_guest("Guest_1".to_string());
    snapshot.local_player = Some(local);
    snapshot.remote_players.clear();
}

fn send_local_intent_to_authoritative(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut api: ResMut<AuthoritativeApi>,
) {
    let direction_x = axis(&keys, KeyCode::KeyA, KeyCode::KeyD);
    let direction_y = axis(&keys, KeyCode::KeyS, KeyCode::KeyW);

    let intent = MovementIntent {
        direction: Vec2f {
            x: direction_x,
            y: direction_y,
        },
        delta_seconds: time.delta_secs(),
    };

    api.move_self(intent);
}

fn tick_authoritative_world(time: Res<Time>, mut api: ResMut<AuthoritativeApi>) {
    api.tick_bots(time.elapsed_secs());
}

fn pull_authoritative_snapshot(api: Res<AuthoritativeApi>, mut snapshot: ResMut<NetworkSnapshot>) {
    let players = api.players_snapshot();
    let local_player_id = api.local_player_id;

    snapshot.local_player = players
        .iter()
        .find(|player| Some(player.id) == local_player_id)
        .cloned();

    snapshot.remote_players = players
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
