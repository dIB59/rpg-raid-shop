//! Bevy game client executable.

mod core;
mod features;
mod module_bindings;

use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkPlugin, LevelSelection};

use crate::core::camera::CameraFollowPlugin;
use crate::core::network::NetworkPlugin;
use crate::core::world;
use crate::features::player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RPG Raid Shop".to_string(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LdtkPlugin)
        .add_plugins((NetworkPlugin, PlayerPlugin, CameraFollowPlugin))
        .add_systems(Startup, world::setup)
        .insert_resource(LevelSelection::index(0))
        .run();
}
