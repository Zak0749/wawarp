use bevy::prelude::*;

#[derive(Component, Default)]
pub enum State {
    On,

    #[default]
    Off,
}
