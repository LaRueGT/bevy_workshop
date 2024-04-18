use bevy::prelude::*;

use crate::game::score::score_resources::*;

pub fn update_score(
    score: Res<Score>,
) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default())
}