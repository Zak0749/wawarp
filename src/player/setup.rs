use super::{player::PlayerBundle, AbilityTimer, AnimationTimer};
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(PlayerBundle {
        sprite: SpriteSheetBundle {
            texture_atlas: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("sprites/player.png"),
                Vec2::new(16., 20.),
                4,
                12,
            )),
            transform: Transform::from_scale(Vec3::splat(3.0)),

            ..default()
        },
        animation_timer: AnimationTimer::new(Timer::from_seconds(0.2, true)),
        ability_timer: AbilityTimer::new(Timer::from_seconds(1., false)),
        ..default()
    });
}
