use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;

mod counter;
mod level;
mod loading_system;
mod spawning_system;

pub use counter::*;
pub use level::*;

use crate::ordering::Stage;
use crate::state::AppState;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TomlAssetPlugin::<Level>::new(&["toml.level"]))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(loading_system::loading_system)
                    .label(Stage::PreUpdate)
                    .before(Stage::Update)
                    .before(Stage::PostUpdate),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(spawning_system::spawning_system)
                    .label(Stage::Update)
                    .after(Stage::PreUpdate)
                    .before(Stage::PostUpdate),
            )
            .init_resource::<Counter>();
    }
}
