use super::{
    ability::{Ability, AbilityState},
    ability_timer::AbilityTimer,
    player::Player,
};
use crate::actions::actions::Actions;
use bevy::prelude::*;

pub fn ability_system(
    time: Res<Time>,
    actions: Res<Actions>,
    mut query: Query<(&mut AbilityState, &mut AbilityTimer), With<Player>>,
) {
    let (mut ability, mut ability_timer) = query.single_mut();

    match ability.as_ref() {
        AbilityState::Preforming(_) if !actions.preforming_ability() => {
            *ability = AbilityState::Cooldown;
            ability_timer.reset();
        }
        _ => (),
    }

    ability_timer.tick(time.delta());

    if actions.preforming_ability() && ability_timer.finished() {
        *ability = AbilityState::Preforming(Ability::Still);
    } else if ability_timer.finished() {
        *ability = AbilityState::Idle;
    }
}
