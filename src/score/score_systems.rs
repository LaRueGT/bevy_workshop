use bevy::prelude::*;

use crate::score::score_components::*;
use crate::score::score_resources::*;

fn update_score(
    score: Res<Score>,
) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}