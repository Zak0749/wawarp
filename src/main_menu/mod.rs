use crate::state::AppState;
use bevy::prelude::*;

mod cleanup;
mod setup;
mod update;

use crate::ordering::Stage;

#[derive(Component, Default)]
pub struct MainMenuIdentifier;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup::setup)
                .label(Stage::PreUpdate)
                .before(Stage::Update)
                .before(Stage::PostUpdate),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu)
                .with_system(cleanup::cleanup)
                .label(Stage::PreUpdate)
                .before(Stage::Update)
                .before(Stage::PostUpdate),
        )
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu)
                .with_system(update::update)
                .label(Stage::Update)
                .after(Stage::PreUpdate)
                .before(Stage::PostUpdate),
        );
    }
}
