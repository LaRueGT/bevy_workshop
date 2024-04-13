pub mod score_components;
pub mod score_systems;
pub mod score_resources;

use bevy::prelude::*;

use crate::score::score_systems::*;
use crate::score::score_components::*;
use crate::score::score_resources::*;

pub struct  ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
        app.add_systems(Update, (
            update_score,
        ),);
    }
}