use bevy::prelude::*;
use wawarp::plugins::{actions::ActionPlugin, camera::CameraPlugin, player::PlayerPlugin};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Wawarp".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(ActionPlugin)
        .run();
}
