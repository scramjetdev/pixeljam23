use bevy::app::{App, Plugin};
use bevy_ecs_ldtk::prelude::LdtkEntityAppExt;
use crate::game::player::components::{PlayerBundle};
use crate::game::player::systems::{animate_player, move_player, spawn_text};

mod systems;
mod components;

pub const PLAYER_SPEED: f32 = 125.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_startup_system(spawn_text)
            .add_system(move_player)
            .add_system(animate_player)
            // .add_startup_system(spawn_player)
            // .add_system(animate_player)
        ;
    }
}