use bevy::prelude::*;
use wawarp::{
    actions::ActionPlugin, camera::CameraPlugin, door::DoorPlugin, level::LevelPlugin,
    main_menu::MainMenuPlugin, player::PlayerPlugin, state::AppState, switch::SwitchPlugin,
    ui::UiPlugin,
};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Wawarp".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::hsl(0., 0., 0.5)))
        .add_state(AppState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(ActionPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(DoorPlugin)
        .add_plugin(SwitchPlugin)
        .run();
}
