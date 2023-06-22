use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod enemy;
mod level;
mod menu;
mod player;
mod types;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(ClearColor(Color::rgb(0.77, 0.93, 0.97)))
        .add_plugin(player::PlayerPlugin)
        .add_plugin(level::LevelPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_startup_system(setup_camera)
        // TODO remove this only for looking around when dev testing
        .add_system(camera_controller)
        .run();
}

#[derive(Component)]
struct GameCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCamera));
}

// Only used to check on colliders and stuff when deving
fn camera_controller(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<GameCamera>>,
) {
    let mut trans = query.single_mut();

    let speed = 50.;

    if keys.pressed(KeyCode::Right) {
        trans.translation.x += 50. * time.delta_seconds() * 3.;
    }

    if keys.pressed(KeyCode::Left) {
        trans.translation.x -= speed * time.delta_seconds() * 3.;
    }

    if keys.pressed(KeyCode::Down) {
        trans.translation.y -= speed * time.delta_seconds() * 3.;
    }

    if keys.pressed(KeyCode::Up) {
        trans.translation.y += speed * time.delta_seconds() * 3.;
    }
}
