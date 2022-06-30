use super::{
    ability::AbilityState, ability_timer::AbilityTimer, animation_timer::AnimationTimer,
    direction::Direction,
};
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    _player: Player,

    direction: Direction,
    animation_timer: AnimationTimer,

    ability: AbilityState,
    ability_timer: AbilityTimer,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}

impl PlayerBundle {
    pub fn new(sprite: SpriteSheetBundle, animation_timer: Timer, ability_timer: Timer) -> Self {
        Self {
            sprite,
            animation_timer: AnimationTimer::new(animation_timer),
            ability_timer: AbilityTimer::new(ability_timer),

            ..default()
        }
    }
}

#[derive(Component, Default)]
pub struct Player {}
