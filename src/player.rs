use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::Color;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::KeyCode::{KeyA, KeyD, KeyS, KeyW};
use bevy::prelude::{
    Camera2d, Commands, Component, KeyCode, Query, Res, Sprite, Time, Transform, With, default,
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

/// Current movement velocity, eased toward the input target for weighty feel.
#[derive(Component, Default)]
struct Velocity(Vec2);

/// Active dodge: remaining seconds and the direction it was locked to on trigger.
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
        Self {
            duration: Timer::from_seconds(0.2, TimerMode::Once),
            cooldown: Timer::from_seconds(1.0, TimerMode::Once),
            dir: Vec2::ZERO,
            speed: 2000.0,
            steer: 1.0,
        }
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

/// Hot-reloadable! Run the app with `dx serve --hot-patch`, then edit the body
#[hot]
fn move_player(
    mut players: Query<(&mut Transform, &mut Velocity, &mut Dodge, &Player)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let (mut transform, mut vel, mut dodge, player) = players.single_mut().unwrap();

    dodge.cooldown.tick(time.delta());
    dodge.duration.tick(time.delta());

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

    // Mid-dodge: dash along dodge.dir, optionally steered by live input, can't re-trigger.
    if !dodge.duration.finished() {
        let heading = (dodge.dir + dir.normalize_or_zero() * dodge.steer).normalize_or_zero();
        let step = heading * dodge.speed * dt;
        transform.translation.x += step.x;
        transform.translation.y += step.y;
        return;
    }

    if keys.just_pressed(KeyCode::Space) && dir != Vec2::ZERO && dodge.cooldown.finished() {
        dodge.cooldown.reset();
        dodge.duration.reset();
        dodge.dir = dir.normalize();
        return;
    }

    // Ease velocity toward the input target: slow to start, quick to stop.
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
