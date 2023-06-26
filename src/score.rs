use bevy::prelude::*;

use crate::state;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_score)
            .insert_resource(Score { val: 0 })
            .add_system(update_score.in_set(OnUpdate(state::AppState::Running)));
    }
}

#[derive(Resource)]
pub struct Score {
    pub val: u32,
}

#[derive(Component)]
struct ScoreText {}

fn setup_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        ScoreText {},
        TextBundle::from_section(
            "912834",
            TextStyle {
                font: asset_server.load("fonts/OpenSans.ttf"),
                font_size: 45.,
                color: Color::BLACK,
            },
        )
        .with_text_alignment(TextAlignment::Right)
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

fn update_score(score: Res<Score>, mut text_q: Query<&mut Text, With<ScoreText>>) {
    let mut text = text_q.single_mut();

    text.sections[0].value = format!("{}", score.val);
}
