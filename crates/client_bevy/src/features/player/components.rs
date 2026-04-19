use bevy::prelude::*;
use shared::PlayerId;

#[derive(Component)]
pub struct LocalPlayerVisual;

#[derive(Component)]
pub(super) struct LocalPlayerOutline;

#[derive(Component)]
pub(super) struct LocalPlayerRenderState {
    pub target_position: Vec2,
    pub visual_position: Vec2,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct RemotePlayerVisual {
    pub id: PlayerId,
}
