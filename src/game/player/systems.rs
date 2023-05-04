use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game::player::components::{AnimationTimer, Player};
use crate::game::player::PLAYER_SPEED;

pub fn move_player(
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut player_transform) = player_transform_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        let mut to_right = 0;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::Q) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
            to_right -= 1;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
            to_right += 1;
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::Z) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();

            if let Ok(mut camera_transform) = camera_query.get_single_mut() {
                camera_transform.translation.x = player_transform.translation.x;
                camera_transform.translation.y = player_transform.translation.y;
            }
        }

        if to_right > 0 && player_transform.is_changed(){
            player_transform.rotation = Quat::from_rotation_y(0.0);
        } else if to_right < 0 {
            player_transform.rotation = Quat::from_rotation_y(3.14);
        }
    }
}

pub fn animate_player(
    time: Res<Time>,
    mut query: Query<
        &mut TextureAtlasSprite,
        With<Player>
    >,
    mut timer_query: Query<&mut AnimationTimer, With<Player>>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut sprite) = query.get_single_mut() {
        let mut timer = timer_query.get_single_mut().unwrap();
        timer.tick(time.delta());

        if let Ok(player_transform) = player_transform_query.get_single_mut() {
            if player_transform.is_changed() {
                if timer.just_finished() {
                    sprite.index = if sprite.index == 7 {
                        0
                    } else {
                        sprite.index + 1
                    };
                }
            } else { sprite.index = 0 }
        }
    }
}

pub fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window, With<PrimaryWindow>>) {
    let _window = window_query.get_single().unwrap();
    commands.spawn(
        TextBundle::from_section(
        "Hi ! I'm Kyle",
        TextStyle {
            font: asset_server.load("fonts/fff_forward.ttf"),
            font_size: 12.0,
            color: Color::WHITE,
        },
        ).with_text_alignment(TextAlignment::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Percent(51.0),
                    left : Val::Percent(51.0),
                    ..default()
                },
                ..default()
            }),
    );
}