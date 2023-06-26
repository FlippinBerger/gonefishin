use bevy::prelude::*;

use crate::state;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(pause_game.in_set(OnUpdate(state::AppState::Running)))
            .add_system(unpause_game.in_set(OnUpdate(state::AppState::Paused)))
            .add_system(spawn_pause_menu.in_schedule(OnEnter(state::AppState::Paused)))
            .add_system(despawn_pause_menu.in_schedule(OnExit(state::AppState::Paused)));
    }
}

fn pause_game(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<state::AppState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(state::AppState::Paused);
    }
}

fn unpause_game(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<state::AppState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(state::AppState::Running);
    }
}

#[derive(Component)]
struct PauseMenu {}

fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            PauseMenu {},
            NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Paused",
                TextStyle {
                    font: asset_server.load("fonts/OpenSans.ttf"),
                    font_size: 45.,
                    color: Color::BLACK,
                },
            ));
        });
}

fn despawn_pause_menu(mut commands: Commands, q: Query<Entity, With<PauseMenu>>) {
    if let Ok(e) = q.get_single() {
        commands.entity(e).despawn_recursive();
    }
}
