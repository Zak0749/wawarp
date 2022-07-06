use bevy::prelude::*;

use crate::switch::{SwitchEvent, SwitchId};

use super::{state::State, Door};

pub fn opening_system(
    mut switch_event: EventReader<SwitchEvent>,
    mut query: Query<(&mut State, &SwitchId), With<Door>>,
) {
    let events = switch_event.iter().map(|v| v.0.clone()).collect::<Vec<_>>();

    for (mut state, id) in query.iter_mut() {
        let e = events.contains(&id);
        if e {
            *state = State::Open;
        } else {
            *state = State::Closed;
        }
    }
}
