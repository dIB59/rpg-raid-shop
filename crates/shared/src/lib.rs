//! Shared protocol and math types used by both the Bevy client and SpacetimeDB module.

use serde::{Deserialize, Serialize};

pub const PLAYER_SPEED_UNITS_PER_SEC: f32 = 180.0;
pub const MAX_SIMULATION_STEP_SECONDS: f32 = 0.1;

/// Stable identifier for a player row in the backend `player` table.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub u64);

/// Lightweight 2D vector used for movement directions and positions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    /// Returns the squared vector length, avoiding a square root.
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Returns a normalized vector, or the zero vector when magnitude is near zero.
    pub fn normalize_or_zero(self) -> Self {
        let len_sq = self.length_squared();
        if len_sq <= f32::EPSILON {
            return Self::default();
        }

        let inv_len = len_sq.sqrt().recip();
        Self {
            x: self.x * inv_len,
            y: self.y * inv_len,
        }
    }

    pub fn scaled(self, factor: f32) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    pub fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// Snapshot representation of a player sent to clients.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlayerState {
    pub id: PlayerId,
    pub name: String,
    pub position: Vec2f,
}

/// Per-frame movement input sampled by the client.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MovementIntent {
    pub direction: Vec2f,
    pub delta_seconds: f32,
}

pub fn clamped_simulation_delta(delta_seconds: f32) -> f32 {
    delta_seconds.clamp(0.0, MAX_SIMULATION_STEP_SECONDS)
}

pub fn simulate_movement(position: Vec2f, direction: Vec2f, delta_seconds: f32) -> Vec2f {
    let clamped_dt = clamped_simulation_delta(delta_seconds);
    if clamped_dt <= f32::EPSILON {
        return position;
    }

    let velocity = direction
        .normalize_or_zero()
        .scaled(PLAYER_SPEED_UNITS_PER_SEC * clamped_dt);
    position.add(velocity)
}
