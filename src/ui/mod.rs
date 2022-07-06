use bevy::prelude::*;
mod setup;
use super::state::AppState;
use crate::ordering::Stage;
use setup::setup;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(setup)
                .label(Stage::PreUpdate)
                .before(Stage::Update)
                .before(Stage::PostUpdate),
        );
    }
}
