mod dodge;
mod player;

use crate::player::PlayerPlugin;
use bevy::prelude::*;
use bevy_simple_subsecond_system::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "RPG Raid Shop".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_plugins(PlayerPlugin)
        .run();
}
