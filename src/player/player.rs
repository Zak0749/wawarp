use crate::switch::SwitchActivator;

use super::{
    ability::{Ability, AbilityState},
    ability_timer::AbilityTimer,
    animation_timer::AnimationTimer,
    direction::Direction,
};
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub _player: Player,

    pub direction: Direction,
    pub animation_timer: AnimationTimer,

    pub ability_state: AbilityState,
    pub ability: Ability,
    pub ability_timer: AbilityTimer,

    pub _button: SwitchActivator,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}

#[derive(Component, Default)]
pub struct Player {}
