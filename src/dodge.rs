use bevy::math::Vec2;
use bevy::prelude::{Component, Timer, TimerMode};
use std::time::Duration;

#[derive(Component)]
pub struct Dodge {
    duration: Timer,
    cooldown: Timer,
    /// Current travel direction; rotates toward live input while dashing.
    dir: Vec2,
    speed: f32,
    /// Max steering speed in radians/sec (0 = locked dash).
    turn_rate: f32,
}

impl Default for Dodge {
    fn default() -> Self {
        let mut duration = Timer::from_seconds(0.2, TimerMode::Once);
        duration.tick(duration.duration());
        let mut cooldown = Timer::from_seconds(1.1, TimerMode::Once);
        cooldown.tick(cooldown.duration());
        Self {
            duration,
            cooldown,
            dir: Vec2::ZERO,
            speed: 2000.0,
            turn_rate: 6.0,
        }
    }
}

impl Dodge {
    pub fn tick(&mut self, dt: Duration) {
        self.duration.tick(dt);
        if self.duration.just_finished() {
            self.cooldown.reset();
        }
        self.cooldown.tick(dt);
    }

    pub fn is_active(&self) -> bool {
        !self.duration.finished()
    }

    fn is_ready(&self) -> bool {
        self.cooldown.finished()
    }

    /// Ask to start a dodge toward `dir`; silently ignored unless one is allowed right now.
    pub fn request(&mut self, dir: Vec2) {
        if self.is_active() || !self.is_ready() || dir == Vec2::ZERO {
            return;
        }
        self.duration.reset();
        self.dir = dir.normalize();
    }

    fn just_ended(&self) -> bool {
        self.duration.just_finished()
    }

    pub fn step(&mut self, input_dir: Vec2, dt: f32) -> Vec2 {
        let input = input_dir.normalize_or_zero();
        if input != Vec2::ZERO {
            let turn = self
                .dir
                .angle_to(input)
                .clamp(-self.turn_rate * dt, self.turn_rate * dt);
            self.dir = Vec2::from_angle(turn).rotate(self.dir);
        }
        self.dir * self.speed * dt
    }
}
