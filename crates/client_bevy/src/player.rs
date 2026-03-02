use bevy::prelude::*;

use crate::network::NetworkSnapshot;

#[derive(Component)]
struct LocalPlayer;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_local_player))
            .add_systems(Update, sync_local_player_transform);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_local_player(mut commands: Commands) {
    commands.spawn((
        LocalPlayer,
        Transform::from_xyz(0.0, 0.0, 100.0),
        GlobalTransform::default(),
    ));
}

fn sync_local_player_transform(
    snapshot: Res<NetworkSnapshot>,
    mut query: Query<&mut Transform, With<LocalPlayer>>,
) {
    let Some(local_state) = &snapshot.local_player else {
        return;
    };

    for mut transform in &mut query {
        transform.translation.x = local_state.position.x;
        transform.translation.y = local_state.position.y;
    }
}
