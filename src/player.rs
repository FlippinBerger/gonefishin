use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use std::time::Duration;

use crate::enemy;
use crate::score;
use crate::state;
use crate::types;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_setup).add_systems(
            (
                animate_sprites,
                player_movement,
                flip_player,
                bomb_drop,
                bomb_movement,
                clear_explosion,
                check_for_fish,
            )
                .in_set(OnUpdate(state::AppState::Running)),
        );
    }
}

#[derive(Component)]
pub struct Player;

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
            transform: Transform::from_xyz(0., player_start, 1.),
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
        // There are other directions, but they don't affect the player
        _ => {}
    }
}

#[derive(Component)]
struct Bomb;

fn bomb_drop(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_q: Query<&Transform, With<Player>>,
    bomb_q: Query<(Entity, &Transform), With<Bomb>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match bomb_q.get_single() {
            // there's already a bomb, so detonate it
            Ok((entity, transform)) => {
                detonate_bomb(commands, meshes, materials, entity, transform);
            }
            // no bombs found from query, so drop one from the player
            Err(_) => {
                let player_transform = player_q.single();

                commands.spawn((
                    Bomb {},
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(12.).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::BLACK)),
                        transform: *player_transform,
                        ..default()
                    },
                    Collider::ball(10.),
                ));
            }
        }
    }
}

fn bomb_movement(time: Res<Time>, mut bomb_q: Query<&mut Transform, With<Bomb>>) {
    if let Ok(mut bomb) = bomb_q.get_single_mut() {
        bomb.translation.y -= 50. * time.delta_seconds() * 3.;
    }
}

#[derive(Component)]
pub struct Explosion {
    timer: Timer,
}

fn detonate_bomb(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    entity: Entity,
    trans: &Transform,
) {
    commands.spawn((
        Explosion {
            timer: Timer::new(Duration::from_millis(250), TimerMode::Once),
        },
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(25.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: *trans,
            ..default()
        },
        Collider::ball(23.),
    ));

    commands.entity(entity).despawn();
}

fn clear_explosion(
    mut commands: Commands,
    time: Res<Time>,
    mut q: Query<(Entity, &mut Explosion)>,
) {
    for (entity, mut explosion) in q.iter_mut() {
        explosion.timer.tick(time.delta());
        if explosion.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

// fn grab_fish(mut commands: Commands) {}

fn check_for_fish(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    rap_ctx: Res<RapierContext>,
    mut score: ResMut<score::Score>,
    fish_q: Query<&enemy::Fish>,
    player_q: Query<&Transform, With<Player>>,
) {
    let player_trans = player_q.single();

    // build the ray to cast
    let ray_pos = Vec2::new(player_trans.translation.x, player_trans.translation.y);
    let ray_dir = Vec2::new(0.0, -1.0);

    let filter = QueryFilter::default();

    if let Some((entity, toi)) = rap_ctx.cast_ray(ray_pos, ray_dir, 32.0, true, filter) {
        let hit_point = ray_pos + ray_dir * toi;
        info!("Entity {:?} hit at point {}", entity, hit_point);

        if let Ok(fish) = fish_q.get(entity) {
            // show the call to action ahove the player
            if keys.just_pressed(KeyCode::A) {
                score.val += enemy::get_score_for_fish_type(&fish.fish_type);
                commands.entity(entity).despawn();
            }
        }
    }
}
