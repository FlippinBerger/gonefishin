use bevy::prelude::*;

use crate::state;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_game_over.in_schedule(OnEnter(state::AppState::GameOver)))
            .add_system(play_again_button.in_set(OnUpdate(state::AppState::GameOver)))
            .add_system(despawn_game_over.in_schedule(OnExit(state::AppState::GameOver)));
    }
}

#[derive(Component)]
struct GameOverMenu {}

#[derive(Component)]
struct RestartButton {}

fn spawn_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            GameOverMenu {},
            NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Game Over",
                TextStyle {
                    font: asset_server.load("fonts/OpenSans.ttf"),
                    font_size: 45.,
                    color: Color::BLACK,
                },
            ));

            parent
                .spawn((
                    ButtonBundle {
                        button: Button {},
                        background_color: BackgroundColor::from(Color::BISQUE),
                        ..default()
                    },
                    RestartButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        style: Style { ..default() },
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play Again",
                                TextStyle {
                                    font: asset_server.load("fonts/OpenSans.ttf"),
                                    font_size: 45.,
                                    color: Color::BLACK,
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}

fn despawn_game_over(mut commands: Commands, q: Query<Entity, With<GameOverMenu>>) {
    if let Ok(e) = q.get_single() {
        commands.entity(e).despawn_recursive();
    }
}

fn play_again_button(
    mut button_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<RestartButton>),
    >,
    mut state: ResMut<NextState<state::AppState>>,
) {
    for (interaction, mut color) in button_q.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                state.set(state::AppState::Running);
            }
            Interaction::Hovered => {
                *color = BackgroundColor::from(Color::LIME_GREEN);
            }
            _ => {
                *color = BackgroundColor::from(Color::WHITE);
            }
        }
    }
}
