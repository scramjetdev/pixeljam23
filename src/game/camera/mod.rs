mod systems;
mod components;

use bevy::app::{App, Plugin};
use crate::game::camera::systems::spawn_camera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_camera)
        ;
    }
}