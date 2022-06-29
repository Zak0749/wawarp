use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,

    #[default]
    None,
}
