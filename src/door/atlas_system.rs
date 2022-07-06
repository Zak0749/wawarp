use super::{state::State, Door};
use bevy::prelude::*;

pub fn atlas_system(mut query: Query<(&mut TextureAtlasSprite, &State), With<Door>>) {
    for (mut sprite, state) in query.iter_mut() {
        sprite.index = match state {
            State::Open => 1,
            State::Closed => 0,
        };
    }
}
