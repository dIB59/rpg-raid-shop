//! Bevy game client executable.

mod camera;
mod module_bindings;
mod network;
mod player;
mod world;

use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkPlugin, LevelSelection};

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
        .add_plugins((
            network::NetworkPlugin,
            player::PlayerPlugin,
            camera::CameraFollowPlugin,
        ))
        .add_systems(Startup, world::setup)
        .insert_resource(LevelSelection::index(0))
        .run();
}
