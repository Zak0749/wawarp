use bevy::prelude::*;

mod id;
mod switch;

pub use id::*;
pub use switch::*;

pub struct SwitchPlugin;

impl Plugin for SwitchPlugin {
    fn build(&self, _app: &mut App) {}
}
