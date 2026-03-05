//! Shared protocol and math types used by both the Bevy client and SpacetimeDB module.

use serde::{Deserialize, Serialize};

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
