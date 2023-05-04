use bevy::prelude::*;
use bevy::time::Timer;
use bevy_ecs_ldtk::LdtkEntity;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    animation_timer: AnimationTimer,
    player: Player,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct AnimationTimer(pub Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}
// #[derive(Component)]
// pub struct AnimationIndices {
//     pub(crate) first: usize,
//     pub(crate) last: usize,
// }