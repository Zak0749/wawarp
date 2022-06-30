use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub enum AbilityState {
    Preforming(Ability),
    Cooldown,

    #[default]
    Idle,
}

#[derive(Debug)]
pub enum Ability {
    Still,
    Forward,
    Backward,
}
