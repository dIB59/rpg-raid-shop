use crate::animation::{Animation, AnimationMode};
use crate::camera::CameraTarget;
use crate::dodge::Dodge;
use crate::movement::{MovementSystems, Velocity};
use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::{AssetServer, Handle};
use bevy::image::Image;
use bevy::input::ButtonInput;
use bevy::math::{UVec2, Vec2};
use bevy::prelude::KeyCode::{KeyA, KeyD, KeyS, KeyW};
use bevy::prelude::{
    Assets, Commands, Component, IntoScheduleConfigs, KeyCode, Query, Res, ResMut,
    TextureAtlasLayout, Time, Transform,
};

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture: Handle<Image> =
        asset_server.load("Tiny Swords/Units/Blue Units/Warrior/Warrior_Idle.png");
    commands.spawn((
        Player::default(),
        Velocity::default(),
        Dodge::default(),
        CameraTarget,
        Transform::default(),
        Animation::from_grid(
            &mut layouts,
            texture,
            UVec2::splat(192),
            8,
            0.09,
            AnimationMode::Repeating,
        ),
    ));
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

fn input_dir(keys: &ButtonInput<KeyCode>) -> Vec2 {
    Vec2::new(
        (keys.pressed(KeyD) as i8 - keys.pressed(KeyA) as i8) as f32,
        (keys.pressed(KeyW) as i8 - keys.pressed(KeyS) as i8) as f32,
    )
}

fn move_player(
    mut players: Query<(&mut Velocity, &mut Dodge, &Player)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let Ok((mut vel, mut dodge, player)) = players.single_mut() else {
        return;
    };

    dodge.tick(time.delta());

    let dir = input_dir(&keys);

    if keys.just_pressed(KeyCode::Space) {
        dodge.request(dir);
    }

    if dodge.is_active() {
        **vel = dodge.velocity(dir, dt);
        return;
    }

    // Coming out of a dash, shed the dash speed so the player doesn't slide.
    if dodge.just_ended() {
        **vel = vel.clamp_length_max(player.speed);
    }

    let target = dir.normalize_or_zero() * player.speed;
    let rate = if target == Vec2::ZERO {
        player.deceleration
    } else {
        player.acceleration
    };
    **vel = vel.move_towards(target, rate * dt);
}

#[derive(Default)]
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, move_player.before(MovementSystems));
    }
}
