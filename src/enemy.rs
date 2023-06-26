use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use rand::Rng;
use std::time::Duration;

use crate::level;
use crate::player;
use crate::state;
use crate::types;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_fish_spawning).add_systems(
            (spawn_fish, fish_collision, fish_swim).in_set(OnUpdate(state::AppState::Running)),
        );
        // .add_system(spawn_fish)
        // .add_system(fish_collision)
        // .add_system(fish_swim);
    }
}

#[derive(Resource)]
struct FishSpawnConfig {
    timer: Timer,
}

#[derive(Debug)]
pub enum FishType {
    Basic,
    Turtle,
}

#[derive(Component)]
pub struct Fish {
    pub fish_type: FishType,
    direction: types::Dir,
}

fn setup_fish_spawning(mut commands: Commands) {
    commands.insert_resource(FishSpawnConfig {
        timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
    })
}

fn spawn_fish(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut config: ResMut<FishSpawnConfig>,
    query: Query<&Window>,
) {
    let window = query.single();
    let window_width = window.width() / 2.;
    let window_height = window.height() / 2.;

    config.timer.tick(time.delta());

    if config.timer.just_finished() {
        // get a random depth to spawn at
        let mut rng = rand::thread_rng();
        let rand_depth = rng.gen_range(-1. * window_height..150.);

        // spawn on or left or right side randomly
        let rand_dir = rng.gen_range(0..2);

        let direction = if rand_dir == 0 {
            types::Dir::Backward
        } else {
            types::Dir::Forward
        };

        let starting_x = match direction {
            types::Dir::Forward => (window_width * -1.) - 20.,
            _ => window_width + 20.,
        };

        // eventually spawn a random type of fish here
        // can weight these differently
        let rand_type_val = rng.gen_range(0.0..1.);

        let fish_type = if rand_type_val > 0.8 {
            FishType::Turtle
        } else {
            FishType::Basic
        };

        let color = match fish_type {
            FishType::Basic => Color::hex("fc6a03").unwrap(),
            FishType::Turtle => Color::hex("3cb043").unwrap(),
        };

        // spawn fish on a timer
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(15., 10.)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_xyz(starting_x, rand_depth, 1.),
                ..default()
            },
            Fish {
                fish_type,
                direction,
            },
            Collider::cuboid(10., 5.),
        ));
    }
}

fn fish_swim(
    time: Res<Time>,
    mut query: Query<(&Fish, &mut Transform), Without<level::Ground>>,
    ground_q: Query<&Transform, With<level::Ground>>,
) {
    let ground_trans = ground_q.single();

    for (fish, mut transform) in query.iter_mut() {
        match fish.direction {
            types::Dir::Forward => transform.translation.x += 50. * time.delta_seconds() * 2.,
            types::Dir::Backward => transform.translation.x -= 50. * time.delta_seconds() * 2.,
            types::Dir::Up => {
                if transform.translation.y <= ground_trans.translation.y {
                    transform.translation.y += 50. * time.delta_seconds() * 2.
                }
            }
        }
    }
}

pub fn get_score_for_fish_type(ft: &FishType) -> u32 {
    match ft {
        FishType::Basic => 100,
        _ => 0,
    }
}

fn fish_collision(
    rap_ctx: Res<RapierContext>,
    explosion_q: Query<Entity, With<player::Explosion>>,
    mut fish_q: Query<&mut Fish>,
) {
    for explosion_entity in explosion_q.iter() {
        for contact_pair in rap_ctx.contacts_with(explosion_entity) {
            let other_entity = if contact_pair.collider1() == explosion_entity {
                contact_pair.collider2()
            } else {
                contact_pair.collider1()
            };

            if let Ok(mut fish) = fish_q.get_mut(other_entity) {
                info!("killing fish of type {:?}", fish.fish_type);

                match fish.fish_type {
                    FishType::Basic => {
                        fish.direction = types::Dir::Up;
                    }
                    FishType::Turtle => {
                        // end the game here
                    }
                }
            } else {
                warn!("couldn't get the fish");
            }
        }
    }
}
