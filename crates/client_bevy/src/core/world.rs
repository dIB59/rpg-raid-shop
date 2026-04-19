use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
};
use bevy_ecs_ldtk::LdtkWorldBundle;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("world.ldtk").into(),
        ..Default::default()
    });
}
