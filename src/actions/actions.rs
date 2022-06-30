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
pub struct Actions(Vec<Action>);

impl Actions {
    pub fn preforming(&self, action: Action) -> bool {
        self.0.contains(&action)
    }

    pub fn preforming_ability(&self) -> bool {
        self.0.contains(&Action::AbilityStill)
            || self.0.contains(&Action::AbilityBackward)
            || self.0.contains(&Action::AbilityForward)
    }

    pub fn set_preforming(&mut self, action: Action, state: bool) {
        if state {
            self.0.push(action);
        } else {
            self.0.retain(|a| a != &action);
        }
    }
}
