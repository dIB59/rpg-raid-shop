use std::time::Duration;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::Color;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{default, Camera2d, Commands, Component, KeyCode, Query, Res, Sprite, Time, Transform, With};
use bevy::prelude::KeyCode::{KeyA, KeyD, KeyS, KeyW};
use bevy::time::{Timer, TimerMode};
use bevy_simple_subsecond_system::hot;

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Player,
        Velocity::default(),
        Dodge {
            duration: Timer::from_seconds(0.2, TimerMode::Once),
            cooldown: Timer::from_seconds(1.0, TimerMode::Once),
            ..default()
        },
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
#[derive(Component, Default)]
struct Dodge {
    duration: Timer,
    cooldown: Timer,
    dir: Vec2,
}

impl Dodge {

}

/// Hot-reloadable! Run the app with `dx serve --hot-patch`, then edit the body
#[hot]
fn move_player(
    mut players: Query<(&mut Transform, &mut Velocity, &mut Dodge), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    const SPEED: f32 = 600.0; // units/sec, top speed
    const ACCEL: f32 = 6000.0; // units/sec^2, ~0.1s to reach top speed
    const DECEL: f32 = 9000.0; // units/sec^2, faster stop than start = weight without mush
    const DODGE_SPEED: f32 = 2000.0; // units/sec
    const DODGE_STEER: f32 = 1.0; // 0 = locked direction, higher = more mid-dodge control

    let dt = time.delta_secs();
    let (mut transform, mut vel, mut dodge) = players.single_mut().unwrap();

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
        let heading = (dodge.dir + dir.normalize_or_zero() * DODGE_STEER).normalize_or_zero();
        let step = heading * DODGE_SPEED * dt;
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
    let target = dir.normalize_or_zero() * SPEED;
    let rate = if target == Vec2::ZERO { DECEL } else { ACCEL };
    let step = (target - vel.0).clamp_length_max(rate * dt);
    vel.0 += step;

    transform.translation.x += vel.0.x * dt;
    transform.translation.y += vel.0.y * dt;
}

#[derive(Default)]
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, move_player);
    }
}
