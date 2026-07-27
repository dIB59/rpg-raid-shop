use bevy::app::{App, Plugin, Update};
use bevy::math::Vec2;
use bevy::prelude::{
    Component, Deref, DerefMut, IntoScheduleConfigs, Query, Res, SystemSet, Transform,
};
use bevy::time::Time;

/// Units per second. Integrated into [`Transform`] by `apply_velocity`.
#[derive(Component, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

/// Anything that writes [`Velocity`] must be scheduled `.before` this set.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MovementSystems;

fn apply_velocity(time: Res<Time>, mut q: Query<(&mut Transform, &Velocity)>) {
    let dt = time.delta_secs();
    for (mut transform, velocity) in &mut q {
        transform.translation += velocity.extend(0.0) * dt;
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_velocity.in_set(MovementSystems));
    }
}
