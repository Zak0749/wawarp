use super::player::PlayerBundle;
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(PlayerBundle::new(
        SpriteSheetBundle {
            texture_atlas: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("player.png"),
                Vec2::new(16., 20.),
                4,
                12,
            )),
            transform: Transform::from_scale(Vec3::splat(3.0)),
            ..default()
        },
        Timer::from_seconds(0.2, true),
        Timer::from_seconds(1., false),
    ));
}
