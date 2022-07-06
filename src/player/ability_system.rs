use super::{ability::AbilityState, ability_timer::AbilityTimer, player::Player};
use crate::actions::actions::{Action, Actions};
use bevy::prelude::*;

pub fn ability_system(
    time: Res<Time>,
    actions: Res<Actions>,
    mut query: Query<(&mut AbilityState, &mut AbilityTimer), With<Player>>,
) {
    let (mut ability_state, mut ability_timer) = query.single_mut();

    if matches!(ability_state.as_ref(), AbilityState::Preforming)
        && !actions.preforming(Action::AbilityPreform)
    {
        *ability_state = AbilityState::Cooldown;
        ability_timer.reset();
    }

    ability_timer.tick(time.delta());

    if actions.preforming(Action::AbilityPreform) && ability_timer.finished() {
        *ability_state = AbilityState::Preforming;
    } else if ability_timer.finished() {
        *ability_state = AbilityState::Idle;
    }
}
