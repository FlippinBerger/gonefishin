use bevy::prelude::*;

use crate::state;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_game_over.in_schedule(OnEnter(state::AppState::GameOver)))
            .add_system(despawn_game_over.in_schedule(OnExit(state::AppState::GameOver)));
    }
}

#[derive(Component)]
struct GameOverMenu {}

fn spawn_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        GameOverMenu {},
        TextBundle::from_section(
            "Game Over",
            TextStyle {
                font: asset_server.load("fonts/OpenSans.ttf"),
                font_size: 45.,
                color: Color::BLACK,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(20.),
                right: Val::Px(20.),
                ..default()
            },
            ..default()
        }),
    ));
}

fn despawn_game_over(mut commands: Commands, q: Query<Entity, With<GameOverMenu>>) {
    if let Ok(e) = q.get_single() {
        commands.entity(e).despawn_recursive();
    }
}

// fn show_game_over(mut q: Query<&mut Visibility, With<GameOverMenu>>) {
//     let mut vis = q.single_mut();
//     *vis = Visibility::Visible;
// }

// fn hide_game_over(mut q: Query<&mut Visibility, With<GameOverMenu>>) {
//     let mut vis = q.single_mut();
//     *vis = Visibility::Hidden;
// }

// fn game_over()
// fn setup_score(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn((
//         ScoreText {},
//         TextBundle::from_section(
//             "912834",
//             TextStyle {
//                 font: asset_server.load("fonts/OpenSans.ttf"),
//                 font_size: 45.,
//                 color: Color::BLACK,
//             },
//         )
//         .with_text_alignment(TextAlignment::Right)
//         .with_style(Style {
//             position_type: PositionType::Absolute,
//             position: UiRect {
//                 top: Val::Px(20.),
//                 right: Val::Px(20.),
//                 ..default()
//             },
//             ..default()
//         }),
//     ));
// }
