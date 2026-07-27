mod animation;
mod camera;
mod character;
mod dodge;
mod faction;
mod health;
mod movement;
mod non_player_character;
mod player;

use crate::animation::AnimationPlugin;
use crate::camera::GameCameraPlugin;
use crate::movement::MovementPlugin;
use crate::non_player_character::NonPlayerCharacterPlugin;
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
        .add_plugins(NonPlayerCharacterPlugin)
        .run();
}
