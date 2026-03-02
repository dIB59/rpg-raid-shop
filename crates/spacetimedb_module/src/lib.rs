use shared::{MovementIntent, PlayerId, PlayerState, Vec2f};
use spacetimedb::Identity;

#[derive(Clone, Debug)]
pub struct PlayerRow {
    pub identity: Identity,
    pub state: PlayerState,
}

#[derive(Default)]
pub struct PrototypeModule {
    pub players: Vec<PlayerRow>,
}

impl PrototypeModule {
    pub fn connect_guest(&mut self, identity: Identity, guest_name: String) -> PlayerState {
        let state = PlayerState {
            id: PlayerId((self.players.len() as u64) + 1),
            name: guest_name,
            position: Vec2f::default(),
        };

        self.players.push(PlayerRow {
            identity,
            state: state.clone(),
        });

        state
    }

    pub fn apply_movement(
        &mut self,
        identity: Identity,
        intent: MovementIntent,
    ) -> Option<PlayerState> {
        let speed = 180.0;
        let max_step = 1.0 / 10.0;
        let clamped_dt = intent.delta_seconds.clamp(0.0, max_step);
        let direction = intent.direction.normalize_or_zero();
        let delta = direction.scaled(speed * clamped_dt);

        let row = self
            .players
            .iter_mut()
            .find(|player| player.identity == identity)?;
        row.state.position.x += delta.x;
        row.state.position.y += delta.y;

        Some(row.state.clone())
    }
}
