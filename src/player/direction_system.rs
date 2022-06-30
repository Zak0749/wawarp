use super::{direction::Direction, player::Player};
use crate::actions::actions::{Action, Actions};
use bevy::prelude::*;

pub fn direction_system(actions: Res<Actions>, mut player: Query<&mut Direction, With<Player>>) {
    let mut direction = player.single_mut();

    *direction = if actions.preforming(Action::Up) {
        Direction::Up
    } else if actions.preforming(Action::Down) {
        Direction::Down
    } else if actions.preforming(Action::Left) {
        Direction::Left
    } else if actions.preforming(Action::Right) {
        Direction::Right
    } else {
        Direction::None
    };
}
