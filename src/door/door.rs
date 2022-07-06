use bevy::prelude::*;

use super::DoorId;
use crate::switch::SwitchId;

#[derive(Bundle, Default)]
pub struct DoorBundle {
    pub _button: Door,

    pub link: SwitchId,
    pub id: DoorId,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}

#[derive(Component, Default)]
pub struct Door;
