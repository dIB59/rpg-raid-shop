use bevy::prelude::*;

/// Component to mark which entity a camera should follow
#[derive(Component)]
pub struct FollowTarget {
    pub entity: Entity,
    pub smoothness: f32, // 0 = instant, higher = smoother
}

/// Plugin to handle camera following logic
pub struct CameraFollowPlugin;

impl Plugin for CameraFollowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_follow_system);
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

            cam_transform.translation.x = cam_transform
                .translation
                .x
                .lerp(target_pos.x, follow.smoothness * time.delta_secs());
            cam_transform.translation.y = cam_transform
                .translation
                .y
                .lerp(target_pos.y, follow.smoothness * time.delta_secs());
        }
    }
}
