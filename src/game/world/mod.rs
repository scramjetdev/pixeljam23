use bevy::app::App;
use bevy::prelude::Plugin;
use bevy_ecs_ldtk::LevelSelection;
use crate::game::world::systems::setup_world_level_0;

mod systems;
mod component;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LevelSelection::Index(0))
            .add_startup_system(setup_world_level_0)
        ;
    }
}