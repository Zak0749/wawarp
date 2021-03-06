use bevy::prelude::*;
mod cleanup;
mod setup;
mod transform_system;

use crate::ordering::Stage;
use crate::state::AppState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(setup::setup)
                .label(Stage::PreUpdate)
                .before(Stage::Update)
                .before(Stage::PostUpdate),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InGame)
                .with_system(cleanup::cleanup)
                .label(Stage::PreUpdate)
                .before(Stage::Update)
                .before(Stage::PostUpdate),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(transform_system::transform_system)
                .label(Stage::PostUpdate)
                .after(Stage::Update)
                .after(Stage::PreUpdate),
        );
    }
}
