use bevy::prelude::*;

#[derive(Component, Default)]
pub enum State {
    Open,

    #[default]
    Closed,
}
