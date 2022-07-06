use bevy::prelude::{Bundle, Component, SpriteSheetBundle};

use crate::door::DoorId;

use super::{state::State, SwitchId};

#[derive(Bundle, Default)]
pub struct SwitchBundle {
    pub _switch: Switch,

    pub link: DoorId,
    pub id: SwitchId,
    pub state: State,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}

#[derive(Component, Default)]
pub struct Switch;
