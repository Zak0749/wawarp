use bevy::prelude::*;
mod setup;
mod transform_system;

use setup::setup;
use transform_system::transform_system;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(transform_system);
    }
}
