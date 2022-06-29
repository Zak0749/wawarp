use crate::components::{
    ability::{Ability, AbilityState},
    ability_timer::AbilityTimer,
    animation_timer::AnimationTimer,
    direction::Direction,
    player::Player,
};

use super::{
    super::components::player::PlayerBundle,
    actions::{Action, Actions},
};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(player_direction_system)
            .add_system(player_transform_system)
            .add_system(player_ability_system)
            .add_system(player_atlas_system);
    }
}

fn setup(
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

fn player_direction_system(actions: Res<Actions>, mut player: Query<&mut Direction, With<Player>>) {
    let mut direction = player.single_mut();

    *direction = if actions.preforming(Action::Up) {
        Direction::Up
    } else if actions.preforming(Action::Down) {
        Direction::Down
    } else if actions.preforming(Action::Left) {
        Direction::Left
    } else if actions.preforming(Action::Right) {
        Direction::Right
    } else {
        Direction::None
    };
}

fn player_ability_system(
    time: Res<Time>,
    actions: Res<Actions>,
    mut query: Query<(&mut AbilityState, &mut AbilityTimer), With<Player>>,
) {
    let (mut ability, mut ability_timer) = query.single_mut();

    match ability.as_ref() {
        AbilityState::Preforming(_) if !actions.preforming_ability() => {
            *ability = AbilityState::Cooldown;
            ability_timer.reset();
        }
        _ => (),
    }

    ability_timer.tick(time.delta());

    if actions.preforming_ability() && ability_timer.finished() {
        *ability = AbilityState::Preforming(Ability::Still);
    } else if ability_timer.finished() {
        *ability = AbilityState::Idle;
    }
}

fn player_transform_system(
    time: Res<Time>,
    mut player: Query<(&mut Transform, &Direction), With<Player>>,
) {
    let (mut transform, direction) = player.single_mut();

    transform.translation.x += match direction {
        Direction::Left => -(time.delta_seconds() * 100.0),
        Direction::Right => time.delta_seconds() * 100.0,
        _ => 0.0,
    };

    transform.translation.y += match direction {
        Direction::Down => -(time.delta_seconds() * 100.0),
        Direction::Up => time.delta_seconds() * 100.0,
        _ => 0.0,
    };
}

fn player_atlas_system(
    mut player: Query<
        (
            &mut TextureAtlasSprite,
            &mut AnimationTimer,
            &Direction,
            &AbilityState,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    let (mut sprite, mut timer, direction, ability) = player.single_mut();

    timer.tick(time.delta());

    if timer.just_finished() {
        sprite.index = match direction {
            Direction::Down => (sprite.index + 1) % 4,
            Direction::Up => ((sprite.index + 1) % 4) + 4,
            Direction::Left => ((sprite.index + 1) % 4) + 8,
            Direction::Right => ((sprite.index + 1) % 4) + 12,
            Direction::None => (sprite.index % 16) - ((sprite.index % 16) % 4),
        } + match ability {
            AbilityState::Idle => 0,
            AbilityState::Preforming(_) => 16,
            AbilityState::Cooldown => 32,
        };
    }
}
