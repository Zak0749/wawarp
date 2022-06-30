use bevy::{prelude::*, render::camera::Camera2d};

use crate::player::player::Player;

pub fn transform_system(
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
