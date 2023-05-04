use bevy::prelude::{AssetServer, Commands, Res};
use bevy_ecs_ldtk::LdtkWorldBundle;

pub fn setup_world_level_0(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("my_map.ldtk"),
        ..Default::default()
    });
}