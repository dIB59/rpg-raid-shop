use bevy::prelude::Component;

#[derive(Component)]
pub struct Character {
    pub speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            speed: 300.0,
            acceleration: 5000.0,
            deceleration: 7000.0,
        }
    }
}
