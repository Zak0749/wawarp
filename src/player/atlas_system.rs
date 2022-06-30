use bevy::prelude::*;

use super::{
    ability::AbilityState, animation_timer::AnimationTimer, direction::Direction, player::Player,
};

pub fn atlas_system(
    mut player: Query<
        (
            &mut TextureAtlasSprite,
            &mut AnimationTimer,
            &Direction,
            &AbilityState,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    let (mut sprite, mut timer, direction, ability) = player.single_mut();

    timer.tick(time.delta());

    if timer.just_finished() {
        sprite.index = match direction {
            Direction::Down => (sprite.index + 1) % 4,
            Direction::Up => ((sprite.index + 1) % 4) + 4,
            Direction::Left => ((sprite.index + 1) % 4) + 8,
            Direction::Right => ((sprite.index + 1) % 4) + 12,
            Direction::None => (sprite.index % 16) - ((sprite.index % 16) % 4),
        } + match ability {
            AbilityState::Idle => 0,
            AbilityState::Preforming(_) => 16,
            AbilityState::Cooldown => 32,
        };
    }
}
