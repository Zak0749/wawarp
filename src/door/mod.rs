use bevy::prelude::*;

mod atlas_system;
mod door;
mod id;
mod opening_system;
mod state;

pub use door::*;
pub use id::*;
pub use state::*;

use crate::{ordering::Stage, state::AppState};

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(opening_system::opening_system)
                .label(Stage::PreUpdate)
                .before(Stage::Update)
                .before(Stage::PostUpdate),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(atlas_system::atlas_system)
                .label(Stage::Update)
                .after(Stage::PreUpdate)
                .before(Stage::PostUpdate),
        );
    }
}
