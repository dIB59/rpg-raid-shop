//! Visual systems for local and remote player entities.

use bevy::prelude::*;
use shared::PlayerId;
use std::collections::{HashMap, HashSet};

use crate::network::NetworkSnapshot;

#[derive(Component)]
struct LocalPlayerVisual;

#[derive(Component)]
struct LocalPlayerOutline;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct RemotePlayerVisual {
    id: PlayerId,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera).add_systems(
            Update,
            (
                ensure_local_player_square,
                sync_local_player_transform,
                sync_remote_player_squares,
            ),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn ensure_local_player_square(
    mut commands: Commands,
    snapshot: Res<NetworkSnapshot>,
    existing: Query<Entity, With<LocalPlayerVisual>>,
) {
    let Some(local_player) = &snapshot.local_player else {
        return;
    };

    if !existing.is_empty() {
        return;
    }

    commands
        .spawn((
            Name::new("LocalPlayer"),
            LocalPlayerVisual,
            Sprite::from_color(player_color(local_player.id), Vec2::splat(24.0)),
            Transform::from_xyz(0.0, 0.0, 10.0),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("LocalPlayerOutline"),
                LocalPlayerOutline,
                Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::splat(30.0)),
                Transform::from_xyz(0.0, 0.0, -0.1),
            ));
        });
}

fn sync_local_player_transform(
    snapshot: Res<NetworkSnapshot>,
    mut query: Query<(&mut Transform, &mut Sprite), With<LocalPlayerVisual>>,
) {
    let Some(local_state) = &snapshot.local_player else {
        return;
    };

    for (mut transform, mut sprite) in &mut query {
        transform.translation.x = local_state.position.x;
        transform.translation.y = local_state.position.y;
        sprite.color = player_color(local_state.id);
    }
}

fn sync_remote_player_squares(
    mut commands: Commands,
    snapshot: Res<NetworkSnapshot>,
    mut query: Query<(Entity, &RemotePlayerVisual, &mut Transform)>,
) {
    let remote_by_id: HashMap<PlayerId, _> = snapshot
        .remote_players
        .iter()
        .map(|player| (player.id, player))
        .collect();
    let wanted_ids: HashSet<PlayerId> = remote_by_id.keys().copied().collect();

    let mut existing_entities: HashMap<PlayerId, Entity> = HashMap::new();

    for (entity, visual, mut transform) in &mut query {
        existing_entities.insert(visual.id, entity);

        if let Some(remote_state) = remote_by_id.get(&visual.id) {
            transform.translation.x = remote_state.position.x;
            transform.translation.y = remote_state.position.y;
        }
    }

    for remote_state in &snapshot.remote_players {
        if existing_entities.contains_key(&remote_state.id) {
            continue;
        }

        commands.spawn((
            Name::new(format!("RemotePlayer_{}", remote_state.id.0)),
            RemotePlayerVisual {
                id: remote_state.id,
            },
            Sprite::from_color(player_color(remote_state.id), Vec2::splat(24.0)),
            Transform::from_xyz(remote_state.position.x, remote_state.position.y, 10.0),
        ));
    }

    for (player_id, entity) in existing_entities {
        if !wanted_ids.contains(&player_id) {
            commands.entity(entity).despawn();
        }
    }
}

fn player_color(player_id: PlayerId) -> Color {
    const PALETTE: [Color; 8] = [
        Color::srgb(0.90, 0.35, 0.35),
        Color::srgb(0.35, 0.75, 0.95),
        Color::srgb(0.35, 0.85, 0.45),
        Color::srgb(0.95, 0.75, 0.30),
        Color::srgb(0.80, 0.45, 0.95),
        Color::srgb(0.30, 0.90, 0.85),
        Color::srgb(0.95, 0.55, 0.20),
        Color::srgb(0.55, 0.65, 0.95),
    ];

    let index = (player_id.0 as usize) % PALETTE.len();
    PALETTE[index]
}
