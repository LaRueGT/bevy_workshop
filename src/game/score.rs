pub mod score_components;
pub mod score_systems;
pub mod score_resources;

use bevy::prelude::*;
use crate::AppState;

use crate::game::score::score_systems::*;
use crate::game::score::score_resources::*;
use crate::game::SimulationState;
use crate::systems::setup_camera;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
        app.init_resource::<FinalScore>();
        app.add_systems(Startup, (
            create_high_scores_table.pipe(db_error_handler),
        ),);
        app.add_systems(Update, (
            update_score, )
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running)
            ), );
        app.add_systems(OnEnter(AppState::GameOver), (
            remove_score.after(update_score),
        ));
    }
}