use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_score).add_system(update_score);
    }
}

#[derive(Component)]
struct Score {
    val: i32,
}

fn setup_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Score { val: 0 },
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

fn update_score(mut score_query: Query<&mut Score>, mut text_q: Query<&mut Text, With<Score>>) {
    let mut score = score_query.single_mut();
    let mut text = text_q.single_mut();

    score.val += 10;

    text.sections[0].value = format!("{}", score.val);
}
