use bevy::{
    input::keyboard::{
        Key,
        KeyCode::{KeyA, KeyD, KeyS, KeyW},
    },
    prelude::*,
};
use bevy_simple_subsecond_system::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RPG Raid Shop".into(),
                ..default()
            }),
            ..default()
        }))
        // Enables hot-patching of any system annotated with `#[hot]`.
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

/// Marker for the thing the player controls.
#[hot]
#[derive(Component)]
struct Player;
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Player,
        Dodge::default(),
        Sprite {
            color: Color::srgb(0.3, 0.6, 1.0),
            custom_size: Some(Vec2::splat(64.0)),
            ..default()
        },
        Transform::default(),
    ));
}

/// Active dodge: remaining seconds and the direction it was locked to on trigger.
#[derive(Component, Default)]
struct Dodge {
    timer: f32,
    dir: Vec2,
}

/// Hot-reloadable! Run the app with `dx serve --hot-patch`, then edit the body
#[hot]
fn move_player(
    mut players: Query<(&mut Transform, &mut Dodge), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    const SPEED: f32 = 300.0; // units/sec
    const DODGE_SPEED: f32 = 3500.0; // units/sec
    const DODGE_TIME: f32 = 0.10; // seconds
    const DODGE_STEER: f32 = 0.0; // 0 = locked direction, higher = more mid-dodge control

    let dt = time.delta_secs();
    let (mut transform, mut dodge) = players.single_mut().unwrap();

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
    if dodge.timer > 0.0 {
        dodge.timer -= dt;
        let heading = (dodge.dir + dir.normalize_or_zero() * DODGE_STEER).normalize_or_zero();
        let step = heading * DODGE_SPEED * dt;
        transform.translation.x += step.x;
        transform.translation.y += step.y;
        return;
    }

    if keys.just_pressed(KeyCode::Space) && dir != Vec2::ZERO {
        dodge.timer = DODGE_TIME;
        dodge.dir = dir.normalize();
        return;
    }

    if dir != Vec2::ZERO {
        let step = dir.normalize() * SPEED * dt;
        transform.translation.x += step.x;
        transform.translation.y += step.y;
    }
}
