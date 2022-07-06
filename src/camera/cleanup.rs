use bevy::{prelude::*, render::camera::Camera2d};

pub fn cleanup(mut commands: Commands, query: Query<Entity, With<Camera2d>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
