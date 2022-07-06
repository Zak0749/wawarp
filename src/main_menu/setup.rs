use super::MainMenuIdentifier;
use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut bg: ResMut<ClearColor>) {
    commands.spawn_bundle(UiCameraBundle::default());

    bg.0 = Color::BLACK;

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                margin: Rect {
                    top: Val::Px(0.0),
                    left: Val::Auto,
                    bottom: Val::Px(0.0),
                    right: Val::Auto,
                },
                ..default()
            },
            text: Text::with_section(
                "Press Enter to start".to_string(),
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
        })
        .insert(MainMenuIdentifier);
}
