use crate::dodge::Dodge;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::Color;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::KeyCode::{KeyA, KeyD, KeyS, KeyW};
use bevy::prelude::{
    Camera2d, Commands, Component, KeyCode, Query, Res, Sprite, Time, Transform, default,
};
use bevy_simple_subsecond_system::hot;

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
