use bevy::prelude::*;

mod door;
mod id;

pub use door::*;
pub use id::*;

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, _app: &mut App) {}
}
