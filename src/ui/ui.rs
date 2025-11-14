use bevy::prelude::*;
use crate::components::{Score, ScoreText};

pub fn setup_score_text(mut commands: Commands) {
    commands.spawn((
        ScoreText,
        Text::new("Score: 0"),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));
}

pub fn update_score_text(
    mut query: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>,
) {
    if score.is_changed() {
        for mut text in &mut query {
            text.0 = format!("Score: {}", score.0);
        }
    }
}
