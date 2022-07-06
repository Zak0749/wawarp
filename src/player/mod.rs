mod ability;
mod ability_system;
mod ability_timer;
mod animation_timer;
mod atlas_system;
mod cleanup;
mod direction;
mod direction_system;
mod player;
mod setup;
mod transform_system;

use crate::state::AppState;
pub use ability::*;
pub use ability_timer::*;
pub use animation_timer::*;
use bevy::prelude::*;
pub use direction::*;
pub use player::*;

use crate::ordering::Stage;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
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
                .with_system(direction_system::direction_system)
                .with_system(ability_system::ability_system)
                .with_system(atlas_system::atlas_system)
                .label(Stage::Update)
                .after(Stage::PreUpdate)
                .before(Stage::PostUpdate),
        );
    }
}
