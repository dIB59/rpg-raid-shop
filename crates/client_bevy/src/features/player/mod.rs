//! Local and remote player visuals, input, and render-state smoothing.

pub mod components;
pub mod systems;

pub use components::LocalPlayerVisual;

use bevy::prelude::*;

use crate::core::network::NetworkSet;

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
                systems::despawn_local_player_visual_when_missing,
                systems::ensure_local_player_visual,
                systems::sync_remote_player_squares,
            )
                .chain()
                .in_set(PlayerSet::Lifecycle),
        )
        .add_systems(
            Update,
            (
                systems::sync_local_player_render_state,
                systems::smooth_local_player_visual,
                systems::draw_local_player_authoritative_border,
            )
                .chain()
                .in_set(PlayerSet::Presentation),
        );
    }
}
