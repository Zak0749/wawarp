use bevy::prelude::*;

use super::{DoorId, state::State};
use crate::switch::SwitchId;

#[derive(Bundle, Default)]
pub struct DoorBundle {
    pub _button: Door,

    pub link: SwitchId,
    pub id: DoorId,
    pub state: State,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}

#[derive(Component, Default)]
pub struct Door;
