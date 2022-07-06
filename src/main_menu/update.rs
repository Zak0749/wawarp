use bevy::prelude::*;

use crate::{
    actions::{Action, Actions},
    state::AppState,
};

pub fn update(actions: Res<Actions>, mut app_state: ResMut<State<AppState>>) {
    if actions.preforming(Action::Start) {
        app_state.set(AppState::InGame).unwrap();
    }
}
