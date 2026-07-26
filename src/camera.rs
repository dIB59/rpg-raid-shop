use bevy::app::{PostUpdate, Startup};
use bevy::camera::Camera2d;
use bevy::prelude::{
    Commands, Component, IntoScheduleConfigs, Plugin, Query, Res, Transform, TransformSystems,
    With, Without,
};
use bevy::time::Time;

#[derive(Component)]
pub struct CameraTarget;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraFollow {
    smoothing: f32,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera, CameraFollow { smoothing: 5.0 }));
}

fn follow_target(
    mut cam: Query<(&mut Transform, &CameraFollow), (With<MainCamera>, Without<CameraTarget>)>,
    target: Query<&Transform, With<CameraTarget>>,
    time: Res<Time>,
) {
    let Ok((mut cam_tf, follow)) = Query::single_mut(&mut cam) else {
        return;
    };
    let Ok(target_tf) = Query::single(&target) else {
        return;
    };
    let t = 1.0 - (-follow.smoothing * time.delta_secs()).exp();
    let next = cam_tf
        .translation
        .truncate()
        .lerp(target_tf.translation.truncate(), t);
    cam_tf.translation.x = next.x;
    cam_tf.translation.y = next.y;
}

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_camera).add_systems(
            PostUpdate,
            follow_target.before(TransformSystems::Propagate),
        );
    }
}
