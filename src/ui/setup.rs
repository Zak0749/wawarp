use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "Playing".to_string(),
            TextStyle {
                font: asset_server.load("fonts/game.ttf"),
                font_size: 60.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..default()
    });
}
