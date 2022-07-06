use super::actions::{Action, Actions};
use bevy::prelude::*;

pub fn keyboard_system(keys: Res<Input<KeyCode>>, mut actions: ResMut<Actions>) {
    if !keys.is_changed() {
        return;
    };
    actions.set_preforming(Action::Up, keys.any_pressed([KeyCode::W, KeyCode::Up]));
    actions.set_preforming(Action::Down, keys.any_pressed([KeyCode::S, KeyCode::Down]));
    actions.set_preforming(Action::Left, keys.any_pressed([KeyCode::A, KeyCode::Left]));
    actions.set_preforming(
        Action::Right,
        keys.any_pressed([KeyCode::D, KeyCode::Right]),
    );
    actions.set_preforming(Action::AbilityStill, keys.pressed(KeyCode::Key1));
    actions.set_preforming(Action::AbilityBackward, keys.pressed(KeyCode::Key2));
    actions.set_preforming(Action::AbilityForward, keys.pressed(KeyCode::Key3));
    actions.set_preforming(Action::AbilityPreform, keys.pressed(KeyCode::Q));
    actions.set_preforming(Action::Start, keys.pressed(KeyCode::Return));
    actions.set_preforming(Action::Pause, keys.pressed(KeyCode::Escape));
}
