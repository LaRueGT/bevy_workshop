use bevy::prelude::*;

use crate::game::score::score_resources::*;
use crate::ui::ui_components::FinalScoreText;

pub fn update_final_score_text(
    final_score: Res<FinalScore>,
    mut text_query: Query<&mut Text, With<FinalScoreText>>,
) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("Final Score: {}", final_score.value.to_string());
    }
}
