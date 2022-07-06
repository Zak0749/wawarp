pub mod actions;
mod keyboard_system;

use crate::ordering::Stage;
pub use actions::*;
use bevy::prelude::*;
use keyboard_system::keyboard_system;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new()
                .with_system(keyboard_system)
                .label(Stage::PreUpdate)
                .before(Stage::Update)
                .before(Stage::PostUpdate),
        );
    }
}
