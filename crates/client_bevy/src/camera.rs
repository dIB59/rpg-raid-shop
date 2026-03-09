use bevy::prelude::*;

use crate::player::{LocalPlayerVisual, PlayerSet};

const CAMERA_FOLLOW_SMOOTHNESS: f32 = 8.0;

/// Component to mark which entity a camera should follow
#[derive(Component)]
pub struct FollowTarget {
    pub entity: Entity,
    pub smoothness: f32,
}

/// Plugin to handle camera following logic
pub struct CameraFollowPlugin;

#[derive(Component)]
struct PlayerCamera;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum CameraSet {
    Follow,
}

impl Plugin for CameraFollowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .configure_sets(Update, CameraSet::Follow.after(PlayerSet::Presentation))
            .add_systems(
                Update,
                (sync_camera_follow_target, camera_follow_system)
                    .chain()
                    .in_set(CameraSet::Follow),
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("PlayerCamera"), Camera2d, PlayerCamera));
}

fn sync_camera_follow_target(
    mut commands: Commands,
    cameras: Query<(Entity, Option<&FollowTarget>), With<PlayerCamera>>,
    local_player: Query<Entity, With<LocalPlayerVisual>>,
) {
    let Ok((camera_entity, follow_target)) = cameras.single() else {
        return;
    };

    let Ok(local_player_entity) = local_player.single() else {
        if follow_target.is_some() {
            commands.entity(camera_entity).remove::<FollowTarget>();
        }
        return;
    };

    let needs_update = follow_target
        .map(|follow_target| {
            follow_target.entity != local_player_entity
                || (follow_target.smoothness - CAMERA_FOLLOW_SMOOTHNESS).abs() > f32::EPSILON
        })
        .unwrap_or(true);

    if needs_update {
        commands.entity(camera_entity).insert(FollowTarget {
            entity: local_player_entity,
            smoothness: CAMERA_FOLLOW_SMOOTHNESS,
        });
    }
}

/// System that moves cameras to follow their targets
fn camera_follow_system(
    time: Res<Time>,
    targets: Query<&Transform, Without<FollowTarget>>,
    mut cameras: Query<(&FollowTarget, &mut Transform)>,
) {
    for (follow, mut cam_transform) in &mut cameras {
        if let Ok(target_transform) = targets.get(follow.entity) {
            let target_pos = target_transform.translation;
            let blend = smoothing_blend(follow.smoothness, time.delta_secs());

            cam_transform.translation.x = cam_transform.translation.x.lerp(target_pos.x, blend);
            cam_transform.translation.y = cam_transform.translation.y.lerp(target_pos.y, blend);
        }
    }
}

fn smoothing_blend(smoothness: f32, delta_seconds: f32) -> f32 {
    1.0 - (-smoothness * delta_seconds).exp()
}
