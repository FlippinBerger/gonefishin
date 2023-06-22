use bevy::prelude::*;

use crate::types;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_setup)
            .add_system(animate_sprites)
            .add_system(player_movement)
            .add_system(flip_player);
    }
}

#[derive(Component)]
struct Player {}

#[derive(Component)]
struct Direction {
    dir: types::Dir,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn player_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    query: Query<&Window>,
) {
    let texture_handle = asset_server.load("Free/Main Characters/Ninja Frog/Idle (32x32).png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 11, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let window = query.single();
    let player_start = window.height() / 2. - 150. + 32.;

    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 10 };
    commands.spawn((
        Player {},
        Direction {
            dir: types::Dir::Forward,
        },
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_xyz(0., player_start, 0.),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player_pos_q: Query<&mut Transform, With<Player>>,
) {
    let mut trans = player_pos_q.single_mut();
    let speed = 50.;

    if keys.pressed(KeyCode::Right) {
        trans.translation.x += 50. * time.delta_seconds() * 3.;
    }

    if keys.pressed(KeyCode::Left) {
        trans.translation.x -= speed * time.delta_seconds() * 3.;
    }
}

fn flip_player(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Direction), With<Player>>,
) {
    let (mut transform, mut direction) = query.single_mut();

    match direction.dir {
        types::Dir::Forward => {
            if keys.pressed(KeyCode::Left) {
                direction.dir = types::Dir::Backward;
                transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
            }
        }
        types::Dir::Backward => {
            if keys.pressed(KeyCode::Right) {
                direction.dir = types::Dir::Forward;
                transform.rotation = Quat::default();
            }
        }
    }
}
