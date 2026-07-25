mod player;

use bevy::{
    input::keyboard::KeyCode::{KeyA, KeyD, KeyS, KeyW},
    prelude::*,
};
use bevy_simple_subsecond_system::prelude::*;
use crate::player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RPG Raid Shop".into(),
                ..default()
            }),
            ..default()
        }))
        // Enables hot-patching of any system annotated with `#[hot]`.
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_plugins(PlayerPlugin)
        .run();
}