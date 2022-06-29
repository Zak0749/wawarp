use bevy::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    AbilityStill,
    AbilityBackward,
    AbilityForward,
}

#[derive(Default)]
pub struct Actions {
    preforming: Vec<Action>,
}

impl Actions {
    pub fn preforming(&self, action: Action) -> bool {
        self.preforming.contains(&action)
    }

    pub fn preforming_ability(&self) -> bool {
        self.preforming.contains(&Action::AbilityStill)
            || self.preforming.contains(&Action::AbilityBackward)
            || self.preforming.contains(&Action::AbilityForward)
    }

    fn set_preforming(&mut self, action: Action, state: bool) {
        if state {
            self.preforming.push(action);
        } else {
            self.preforming.retain(|a| a != &action);
        }
    }
}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::new()
                .label("input_systems")
                .with_system(keyboard_action_system),
        );
    }
}

fn keyboard_action_system(keys: Res<Input<KeyCode>>, mut actions: ResMut<Actions>) {
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
}
