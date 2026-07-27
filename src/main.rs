mod animation;
mod camera;
mod dodge;
mod enemy;
mod health;
mod movement;
mod player;

use crate::animation::AnimationPlugin;
use crate::camera::GameCameraPlugin;
use crate::movement::MovementPlugin;
use crate::player::PlayerPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "RPG Raid Shop".into(),
                        name: Some("rpg-raid-shop".to_string()),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(PlayerPlugin)
        .add_plugins(GameCameraPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(MovementPlugin)
        .run();
}
