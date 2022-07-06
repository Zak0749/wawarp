use bevy::prelude::*;

mod activation;
mod activator;
mod atlas_system;
mod event;
mod id;
mod state;
mod switch;

pub use activator::*;
pub use event::*;
pub use id::*;
pub use state::State;
pub use switch::*;

use crate::ordering::Stage;
use crate::state::AppState;

pub struct SwitchPlugin;

impl Plugin for SwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(activation::activation_system)
                .label(Stage::PostUpdate)
                .after(Stage::PreUpdate)
                .after(Stage::Update),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(atlas_system::atlas_system)
                .label(Stage::Update)
                .after(Stage::PreUpdate)
                .before(Stage::PostUpdate),
        )
        .add_event::<SwitchEvent>();
    }
}
