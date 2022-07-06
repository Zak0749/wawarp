use super::{Counter, Level};
use bevy::prelude::*;
pub fn loading_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    counter: Res<Counter>,
    handle: Option<Res<Handle<Level>>>,
) {
    if counter.is_changed() || matches!(handle, None) {
        let path = format!("levels/{}.toml.level", counter.to_string());
        let handle: Handle<Level> = asset_server.load(path.as_str());
        commands.insert_resource(handle);
    }
}
