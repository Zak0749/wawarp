use super::{state::State, Switch};
use bevy::prelude::*;

pub fn atlas_system(mut query: Query<(&mut TextureAtlasSprite, &State), With<Switch>>) {
    for (mut sprite, state) in query.iter_mut() {
        sprite.index = match state {
            State::On => 1,
            State::Off => 0,
        };
    }
}
