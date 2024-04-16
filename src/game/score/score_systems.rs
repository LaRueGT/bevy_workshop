use bevy::prelude::*;

use crate::game::score::score_resources::*;

pub fn update_score(
    score: Res<Score>,
) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}