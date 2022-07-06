use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub enum AbilityState {
    Preforming,
    Cooldown,

    #[default]
    Idle,
}

#[derive(Component, Debug, Default)]
pub enum Ability {
    Still,
    Forward,
    Backward,

    #[default]
    None,
}
