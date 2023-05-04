mod components;
mod systems;
mod player;
mod camera;
mod world;

use bevy::app::{App, Plugin};
use crate::game::camera::CameraPlugin;
use crate::game::player::PlayerPlugin;
use crate::game::world::WorldPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(CameraPlugin)
            .add_plugin(WorldPlugin)
            .add_plugin(PlayerPlugin)
        ;
    }
}