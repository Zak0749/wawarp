use bevy::{prelude::*, render::camera::Camera2d};

use crate::components::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(camera_transform_system);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn camera_transform_system(
    mut set: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<Camera2d>>,
    )>,
) {
    let player_t = set.p0().single_mut().translation;
    let mut camera = set.p1();
    let mut camera = camera.single_mut();
    camera.translation =
        camera.translation + (player_t + Vec3::splat(1.) - camera.translation) * 0.056;
}
