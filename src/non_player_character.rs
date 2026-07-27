use crate::animation::{Animation, AnimationMode};
use crate::character::Character;
use crate::faction::Faction;
use crate::health::Health;
use crate::movement::{MovementSystems, Velocity};
use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::{AssetServer, Assets};
use bevy::image::TextureAtlasLayout;
use bevy::math::{UVec2, Vec2};
use bevy::prelude::{
    Commands, Component, Entity, IntoScheduleConfigs, Query, Res, ResMut, Transform,
};
use bevy::time::Time;

/// Gives a character chase AI. Absent on the player, who is driven by input.
#[derive(Component)]
struct Aggro {
    radius: f32,
}

impl Default for Aggro {
    fn default() -> Self {
        Self { radius: 600.0 }
    }
}

/// Steers every [`Aggro`] character toward the nearest entity it is hostile to.
fn chase(
    time: Res<Time>,
    mut movers: Query<(
        Entity,
        &Transform,
        &Faction,
        &Character,
        &Aggro,
        &mut Velocity,
    )>,
    candidates: Query<(Entity, &Transform, &Faction)>,
) {
    let dt = time.delta_secs();

    for (me, tf, faction, character, aggro, mut vel) in &mut movers {
        let pos = tf.translation.truncate();

        let nearest = candidates
            .iter()
            .filter(|(e, _, f)| *e != me && faction.is_hostile(**f))
            .map(|(_, tf, _)| tf.translation.truncate())
            .min_by(|a, b| a.distance_squared(pos).total_cmp(&b.distance_squared(pos)));

        let target = match nearest {
            Some(t) if t.distance(pos) <= aggro.radius => {
                (t - pos).normalize_or_zero() * character.speed
            }
            _ => Vec2::ZERO,
        };

        // Ease in and out the way the player does rather than snapping to full
        // speed, otherwise hostiles read as noticeably more robotic than he is.
        let rate = if target == Vec2::ZERO {
            character.deceleration
        } else {
            character.acceleration
        };
        **vel = vel.move_towards(target, rate * dt);
    }
}

fn spawn_transform() -> Transform {
    let (x, y) = (
        rand::random_range(-400.0..400.0),
        rand::random_range(-400.0..400.0),
    );
    Transform::from_xyz(x, y, 0.0)
}

fn spawn_non_player_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("Tiny Swords/Units/Red Units/Warrior/Warrior_Idle.png");
    let transform = spawn_transform();
    for _ in 0..3 {
        commands.spawn((
            Character::default(),
            Aggro::default(),
            Faction::Hostile,
            Health(100.0),
            Velocity::default(),
            transform,
            Animation::from_grid(
                &mut layouts,
                texture.clone(),
                UVec2::splat(192),
                8,
                0.09,
                AnimationMode::Repeating,
            ),
        ));
    }
}

pub struct NonPlayerCharacterPlugin;

impl Plugin for NonPlayerCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_non_player_character)
            // `chase` writes `Velocity`, so it owes `MovementSystems` the ordering.
            .add_systems(Update, chase.before(MovementSystems));
    }
}
