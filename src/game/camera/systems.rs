use bevy::prelude::{Camera2dBundle, Commands, default, Query, Window, With};
use bevy::window::PrimaryWindow;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let _window = window_query.get_single().unwrap();
    let mut camera_bundle = Camera2dBundle {
        ..default()
    };
    camera_bundle.projection.scale *= 0.3;
    commands.spawn(camera_bundle);
}