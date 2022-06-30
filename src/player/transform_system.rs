use bevy::prelude::*;

use super::{direction::Direction, player::Player};

pub fn transform_system(
    time: Res<Time>,
    mut player: Query<(&mut Transform, &Direction), With<Player>>,
) {
    let (mut transform, direction) = player.single_mut();

    transform.translation.x += match direction {
        Direction::Left => -(time.delta_seconds() * 100.0),
        Direction::Right => time.delta_seconds() * 100.0,
        _ => 0.0,
    };

    transform.translation.y += match direction {
        Direction::Down => -(time.delta_seconds() * 100.0),
        Direction::Up => time.delta_seconds() * 100.0,
        _ => 0.0,
    };
}
