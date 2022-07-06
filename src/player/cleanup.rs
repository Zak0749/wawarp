use super::Player;
use bevy::prelude::*;

pub fn cleanup(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
