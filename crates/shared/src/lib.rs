//! Shared protocol and math types used by both the Bevy client and SpacetimeDB module.

use serde::{Deserialize, Serialize};
use std::fmt;

pub const PLAYER_SPEED_UNITS_PER_SEC: f32 = 180.0;
pub const MAX_SIMULATION_STEP_SECONDS: f32 = 0.1;
pub const DISPLAY_NAME_MAX_LEN: usize = 24;

/// Stable identifier for a player row in the backend `player` table.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub u64);

/// A validated display name. Once constructed, guaranteed to be non-empty and
/// contain only `[A-Za-z0-9_-]`, trimmed to [`DISPLAY_NAME_MAX_LEN`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DisplayName(String);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NameError {
    Empty,
    NoValidCharacters,
}

impl fmt::Display for NameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "name cannot be empty"),
            Self::NoValidCharacters => write!(f, "name must contain letters or numbers"),
        }
    }
}

impl DisplayName {
    pub fn new(raw: &str) -> Result<Self, NameError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(NameError::Empty);
        }

        let mut normalized = String::with_capacity(trimmed.len().min(DISPLAY_NAME_MAX_LEN));
        for character in trimmed.chars() {
            if character.is_alphanumeric() || character == '_' || character == '-' {
                normalized.push(character);
            }

            if normalized.len() >= DISPLAY_NAME_MAX_LEN {
                break;
            }
        }

        if normalized.is_empty() {
            return Err(NameError::NoValidCharacters);
        }

        Ok(Self(normalized))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

/// Lightweight 2D vector used for movement directions and positions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
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

/// Per-frame movement input. Construct via [`MovementIntent::new`]; the
/// constructor normalizes direction to unit length and clamps delta to
/// [`MAX_SIMULATION_STEP_SECONDS`], so any existing value is already safe to
/// feed into [`simulate_movement`].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MovementIntent {
    direction: Vec2f,
    delta_seconds: f32,
}

impl MovementIntent {
    pub fn new(direction: Vec2f, delta_seconds: f32) -> Self {
        Self {
            direction: direction.normalize_or_zero(),
            delta_seconds: clamped_simulation_delta(delta_seconds),
        }
    }

    pub fn direction(&self) -> Vec2f {
        self.direction
    }

    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    pub fn is_idle(&self) -> bool {
        self.direction.length_squared() <= f32::EPSILON
            || self.delta_seconds <= f32::EPSILON
    }
}

pub fn clamped_simulation_delta(delta_seconds: f32) -> f32 {
    delta_seconds.clamp(0.0, MAX_SIMULATION_STEP_SECONDS)
}

pub fn simulate_movement(position: Vec2f, intent: MovementIntent) -> Vec2f {
    if intent.is_idle() {
        return position;
    }

    let velocity = intent
        .direction
        .scaled(PLAYER_SPEED_UNITS_PER_SEC * intent.delta_seconds);
    position.add(velocity)
}
