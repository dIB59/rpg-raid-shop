//! Visual systems for local and remote player entities.

use bevy::prelude::*;
use shared::PlayerId;
use std::collections::{HashMap, HashSet};

use crate::network::{LocalPlayerPrediction, NetworkSet, NetworkSnapshot};

const LOCAL_PLAYER_VISUAL_SMOOTHNESS: f32 = 18.0;
const LOCAL_PLAYER_SPRITE_SIZE: f32 = 24.0;
const AUTHORITATIVE_BORDER_COLOR: Color = Color::srgb(1.0, 0.85, 0.2);

#[derive(Component)]
pub(crate) struct LocalPlayerVisual;

#[derive(Component)]
struct LocalPlayerOutline;

#[derive(Component)]
struct LocalPlayerRenderState {
    target_position: Vec2,
    visual_position: Vec2,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct RemotePlayerVisual {
    id: PlayerId,
}

pub struct PlayerPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerSet {
    Lifecycle,
    Presentation,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                PlayerSet::Lifecycle.after(NetworkSet::Sync),
                PlayerSet::Presentation.after(PlayerSet::Lifecycle),
            ),
        )
        .add_systems(
            Update,
            (
                despawn_local_player_visual_when_missing,
                ensure_local_player_visual,
                sync_remote_player_squares,
            )
                .chain()
                .in_set(PlayerSet::Lifecycle),
        )
        .add_systems(
            Update,
            (
                sync_local_player_render_state,
                smooth_local_player_visual,
                draw_local_player_authoritative_border,
            )
                .chain()
                .in_set(PlayerSet::Presentation),
        );
    }
}

fn despawn_local_player_visual_when_missing(
    mut commands: Commands,
    snapshot: Res<NetworkSnapshot>,
    existing: Query<Entity, With<LocalPlayerVisual>>,
) {
    if snapshot.local_player.is_some() {
        return;
    }

    for entity in &existing {
        commands.entity(entity).despawn();
    }
}

fn ensure_local_player_visual(
    mut commands: Commands,
    snapshot: Res<NetworkSnapshot>,
    prediction: Res<LocalPlayerPrediction>,
    existing: Query<Entity, With<LocalPlayerVisual>>,
) {
    let Some(local_player) = &snapshot.local_player else {
        return;
    };

    if !existing.is_empty() {
        return;
    }

    let initial_position = prediction
        .predicted_position()
        .unwrap_or(local_player.position);
    let initial_position = vec2(initial_position.x, initial_position.y);

    commands
        .spawn((
            Name::new("LocalPlayer"),
            LocalPlayerVisual,
            LocalPlayerRenderState {
                target_position: initial_position,
                visual_position: initial_position,
            },
            Sprite::from_color(
                player_color(local_player.id),
                Vec2::splat(LOCAL_PLAYER_SPRITE_SIZE),
            ),
            Transform::from_xyz(initial_position.x, initial_position.y, 10.0),
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

fn sync_local_player_render_state(
    snapshot: Res<NetworkSnapshot>,
    prediction: Res<LocalPlayerPrediction>,
    mut query: Query<(&mut LocalPlayerRenderState, &mut Sprite), With<LocalPlayerVisual>>,
) {
    let Some(local_state) = &snapshot.local_player else {
        return;
    };

    let target_position = prediction
        .predicted_position()
        .unwrap_or(local_state.position);
    let target_position = vec2(target_position.x, target_position.y);

    for (mut render_state, mut sprite) in &mut query {
        render_state.target_position = target_position;
        sprite.color = player_color(local_state.id);
    }
}

fn smooth_local_player_visual(
    time: Res<Time>,
    mut query: Query<(&mut LocalPlayerRenderState, &mut Transform), With<LocalPlayerVisual>>,
) {
    let blend = smoothing_blend(LOCAL_PLAYER_VISUAL_SMOOTHNESS, time.delta_secs());

    for (mut render_state, mut transform) in &mut query {
        render_state.visual_position = render_state
            .visual_position
            .lerp(render_state.target_position, blend);
        transform.translation.x = render_state.visual_position.x;
        transform.translation.y = render_state.visual_position.y;
    }
}

fn draw_local_player_authoritative_border(
    snapshot: Res<NetworkSnapshot>,
    mut gizmos: Gizmos,
) {
    let Some(local_state) = &snapshot.local_player else {
        return;
    };

    let position = vec2(local_state.position.x, local_state.position.y);
    gizmos.rect_2d(
        position,
        Vec2::splat(LOCAL_PLAYER_SPRITE_SIZE),
        AUTHORITATIVE_BORDER_COLOR,
    );
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

fn smoothing_blend(smoothness: f32, delta_seconds: f32) -> f32 {
    1.0 - (-smoothness * delta_seconds).exp()
}
