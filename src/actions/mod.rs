pub mod actions;
mod keyboard_system;

use actions::Actions;
use bevy::prelude::*;
use keyboard_system::keyboard_system;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::new()
                .label("input_systems")
                .with_system(keyboard_system),
        );
    }
}
