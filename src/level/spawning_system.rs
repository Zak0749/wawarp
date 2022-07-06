use bevy::prelude::*;

use super::{level::ComponentType, Level};
use crate::{door::DoorBundle, switch::SwitchBundle};

pub fn spawning_system(
    mut commands: Commands,
    handle: Option<Res<Handle<Level>>>,
    level: Option<Res<Assets<Level>>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut bg: ResMut<ClearColor>,
) {
    if let Some(level) = level {
        if let Some(handle) = handle {
            if level.is_changed() {
                if let Some(level) = level.get(handle.id) {
                    bg.0 = Color::hex(level.background.clone()).unwrap();
                    for component in level.components.iter() {
                        match component.component {
                            ComponentType::Button => commands.spawn_bundle(SwitchBundle {
                                sprite: SpriteSheetBundle {
                                    texture_atlas: texture_atlases.add(TextureAtlas::from_grid(
                                        asset_server.load("sprites/button.png"),
                                        Vec2::new(16., 16.),
                                        2,
                                        1,
                                    )),
                                    transform: Transform {
                                        translation: Vec3::new(
                                            component.x as f32 * 50.,
                                            component.y as f32 * 50.,
                                            -1.0,
                                        ),
                                        scale: Vec3::splat(3.0),
                                        ..default()
                                    },

                                    ..default()
                                },
                                ..default()
                            }),
                            ComponentType::Door => commands.spawn_bundle(DoorBundle {
                                sprite: SpriteSheetBundle {
                                    texture_atlas: texture_atlases.add(TextureAtlas::from_grid(
                                        asset_server.load("sprites/door.png"),
                                        Vec2::new(16., 16.),
                                        2,
                                        1,
                                    )),
                                    transform: Transform {
                                        translation: Vec3::new(
                                            component.x as f32 * 100.,
                                            component.y as f32 * 100.,
                                            -1.0,
                                        ),
                                        scale: Vec3::splat(3.0),
                                        ..default()
                                    },

                                    ..default()
                                },
                                ..default()
                            }),
                        };
                    }
                }
            }
        }
    }
}
