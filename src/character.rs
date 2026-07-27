use crate::animation::{Animation, AnimationMode};
use crate::faction::Faction;
use crate::movement::Velocity;
use bevy::asset::{AssetServer, Assets};
use bevy::image::TextureAtlasLayout;
use bevy::math::{UVec2, Vec2};
use bevy::prelude::{Commands, Component, Entity, Query, Res, ResMut, Transform};
use rand::random;

#[derive(Component)]
struct Character {
    pub speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub aggro_radius: f32,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            speed: 300.0,
            acceleration: 5000.0,
            deceleration: 7000.0,
            aggro_radius: 600.0,
        }
    }
}
fn chase(
    mut movers: Query<(Entity, &Transform, &Faction, &mut Velocity, &Character)>,
    candidates: Query<(Entity, &Transform, &Faction)>,
) {
    for (me, tf, faction, mut vel, character) in &mut movers {
        let pos = tf.translation.truncate();

        let nearest = candidates
            .iter()
            .filter(|(e, _, f)| *e != me && faction.is_hostile(**f))
            .map(|(_, tf, _)| tf.translation.truncate())
            .min_by(|a, b| a.distance_squared(pos).total_cmp(&b.distance_squared(pos)));

        let Some(target) = nearest else {
            vel.0 = Vec2::ZERO;
            continue;
        };

        let to_target = target - pos;
        vel.0 = if to_target.length() > character.aggro_radius {
            Vec2::ZERO
        } else {
            to_target.normalize_or_zero() * character.speed
        };
    }
}

fn spawn_transform() -> Transform {
    let (x, y) = (
        rand::random_range(-400.0..400.0),
        rand::random_range(-400.0..400.0),
    );
    Transform::from_xyz(x, y, 0.0)
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("Tiny Swords/Units/Red Units/Warrior/Warrior_Idle.png");
    commands.spawn((
        Character::default(),
        spawn_transform(),
        Faction::Hostile,
        Animation::from_grid(
            &mut layouts,
            texture,
            UVec2::splat(192),
            8,
            0.09,
            AnimationMode::Repeating,
        ),
        Velocity::default(),
    ));
}
