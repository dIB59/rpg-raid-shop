mod module_bindings;
mod network;
mod player;

use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkPlugin;

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
        .add_plugins((network::NetworkPlugin, player::PlayerPlugin))
        .run();
}
