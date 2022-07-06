use super::MainMenuIdentifier;
use bevy::prelude::*;

pub fn cleanup(mut commands: Commands, query: Query<Entity, With<MainMenuIdentifier>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
