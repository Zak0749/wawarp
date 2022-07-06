use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum Stage {
    PreUpdate,
    Update,
    PostUpdate,
}
