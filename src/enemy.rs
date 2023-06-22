use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use rand::Rng;
use std::time::Duration;

use crate::types;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_fish_spawning)
            .add_system(spawn_fish)
            .add_system(fish_swim);
    }
}

#[derive(Resource)]
struct FishSpawnConfig {
    timer: Timer,
}

enum FishType {
    Basic,
}

#[derive(Component)]
struct Fish {
    fish_type: FishType,
    direction: types::Dir,
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
        // let random_depth = 20.;

        let mut rng = rand::thread_rng();
        let random_depth = rng.gen_range(-1. * window_height..150.);

        // spawn on or left or right side randomly
        let direction = types::Dir::Backward;

        let starting_x = match direction {
            types::Dir::Forward => (window_width * -1.) - 20.,
            types::Dir::Backward => window_width + 20.,
        };

        // eventually spawn a random type of fish here
        // can weight these differently
        let fish_type = FishType::Basic;

        // spawn fish on a timer
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(15., 10.)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::hex("fc6a03").unwrap())),
                transform: Transform::from_xyz(starting_x, random_depth, 1.),
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

fn fish_swim(time: Res<Time>, mut query: Query<(&Fish, &mut Transform)>) {
    for (fish, mut transform) in query.iter_mut() {
        match fish.direction {
            types::Dir::Forward => transform.translation.x += 50. * time.delta_seconds() * 2.,
            types::Dir::Backward => transform.translation.x -= 50. * time.delta_seconds() * 2.,
        }
    }
}

fn fish_despawn(
    mut commands: Commands,
    mut query: Query<(Entity, &Fish, &Transform, &ComputedVisibility)>,
) {
    for (entity, fish, trans, vis) in query.iter_mut() {
        match fish.direction {
            types::Dir::Forward => {
                if trans.translation.x > 0. && !vis.is_visible_in_view() {
                    commands.entity(entity).despawn();
                }
            }
            types::Dir::Backward => {
                if trans.translation.x < 0. && vis.is_visible_in_view() {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn setup_fish_spawning(mut commands: Commands) {
    commands.insert_resource(FishSpawnConfig {
        timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
    })
}
