use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::Color;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::KeyCode::{KeyA, KeyD, KeyS, KeyW};
use bevy::prelude::{
    Camera2d, Commands, Component, KeyCode, Query, Res, Sprite, Time, Transform, default,
};
use bevy::time::{Timer, TimerMode};
use bevy_simple_subsecond_system::hot;
use std::time::Duration;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Player::default(),
        Velocity::default(),
        Dodge::default(),
        Sprite {
            color: Color::srgb(0.3, 0.6, 1.0),
            custom_size: Some(Vec2::splat(64.0)),
            ..default()
        },
        Transform::default(),
    ));
}

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component)]
struct Dodge {
    duration: Timer,
    cooldown: Timer,
    dir: Vec2,
    speed: f32,
    steer: f32,
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
            steer: 1.0,
        }
    }
}

impl Dodge {
    fn tick(&mut self, dt: Duration) {
        self.duration.tick(dt);
        if self.duration.just_finished() {
            self.cooldown.reset();
        }
        self.cooldown.tick(dt);
    }

    fn is_active(&self) -> bool {
        !self.duration.finished()
    }

    fn is_ready(&self) -> bool {
        self.cooldown.finished()
    }

    /// Ask to start a dodge toward `dir`; silently ignored unless one is allowed right now.
    fn request(&mut self, dir: Vec2) {
        if self.is_active() || !self.is_ready() || dir == Vec2::ZERO {
            return;
        }
        self.duration.reset();
        self.dir = dir.normalize();
    }

    fn just_ended(&self) -> bool {
        self.duration.just_finished()
    }

    fn step(&mut self, input_dir: Vec2, dt: f32) -> Vec2 {
        let heading = (self.dir + input_dir.normalize_or_zero() * self.steer).normalize_or_zero();
        heading * self.speed * dt
    }
}

#[derive(Component)]
struct Player {
    pub speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 600.0,
            acceleration: 6000.0,
            deceleration: 9000.0,
        }
    }
}

#[hot]
fn move_player(
    mut players: Query<(&mut Transform, &mut Velocity, &mut Dodge, &Player)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let (mut transform, mut vel, mut dodge, player) = players.single_mut().unwrap();

    dodge.tick(time.delta());

    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyW) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyD) {
        dir.x += 1.0;
    }
    if keys.pressed(KeyS) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyA) {
        dir.x -= 1.0;
    }

    if keys.just_pressed(KeyCode::Space) {
        dodge.request(dir);
    }

    if dodge.is_active() {
        let step = dodge.step(dir, dt);
        transform.translation.x += step.x;
        transform.translation.y += step.y;
        return;
    }

    let target = dir.normalize_or_zero() * player.speed;
    let rate = if target == Vec2::ZERO {
        player.deceleration
    } else {
        player.acceleration
    };
    let step = (target - vel.0).clamp_length_max(rate * dt);
    vel.0 += step;

    transform.translation.x += vel.0.x * dt;
    transform.translation.y += vel.0.y * dt;
}

#[derive(Default)]
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, move_player);
    }
}
