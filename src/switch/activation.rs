use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use super::{state::State, Switch, SwitchActivator, SwitchEvent, SwitchId};

pub fn activation_system(
    mut buttons: Query<(&Transform, &mut State, &SwitchId), With<Switch>>,
    activators: Query<&Transform, With<SwitchActivator>>,
    mut switch_event: EventWriter<SwitchEvent>,
) {
    for (button_transform, mut state, id) in buttons.iter_mut() {
        *state = State::Off;
        for activator_transform in activators.iter() {
            if collide(
                button_transform.translation,
                Vec2::new(16., 16.),
                activator_transform.translation,
                Vec2::new(16., 20.),
            )
            .is_some()
            {
                *state = State::On;
                switch_event.send(SwitchEvent(id.clone()));
            }
        }
    }
}
