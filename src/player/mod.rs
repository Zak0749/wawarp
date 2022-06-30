pub mod ability;
mod ability_system;
pub mod ability_timer;
pub mod animation_timer;
mod atlas_system;
pub mod direction;
mod direction_system;
pub mod player;
mod setup;
mod transform_system;

use ability_system::ability_system;
use atlas_system::atlas_system;
use bevy::prelude::*;
use direction_system::direction_system;
use setup::setup;
use transform_system::transform_system;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(direction_system)
            .add_system(transform_system)
            .add_system(ability_system)
            .add_system(atlas_system);
    }
}
