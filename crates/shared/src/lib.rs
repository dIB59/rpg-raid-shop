use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub u64);

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlayerState {
    pub id: PlayerId,
    pub name: String,
    pub position: Vec2f,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MovementIntent {
    pub direction: Vec2f,
    pub delta_seconds: f32,
}
