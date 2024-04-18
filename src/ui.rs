mod ui_systems;
mod ui_components;
mod ui_styles;

//extern
use bevy::prelude::*;
use crate::AppState;
use crate::ui::ui_systems::ui_interactions::*;
use crate::ui::ui_systems::ui_layout::*;
use crate::ui::ui_systems::ui_updates::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        //main menu
        app
            .add_systems(OnEnter(AppState::MainMenu), (
                spawn_main_menu,
            ), )
            .add_systems(Update, (
                interact_with_play_button,
                interact_with_quit_button,
            )
                .run_if(in_state(AppState::MainMenu)), )
            .add_systems(OnExit(AppState::MainMenu), (
                despawn_main_menu,
            ), );
        //game over menu
        app
            .add_systems(OnEnter(AppState::GameOver), (
                spawn_game_over_menu,
            ), )
            .add_systems(Update, (
                interact_with_restart_button,
                interact_with_main_menu_button,
                interact_with_quit_button,
                update_final_score_text,
            )
                .run_if(in_state(AppState::GameOver)), )
            .add_systems(OnExit(AppState::GameOver), (
                despawn_game_over_menu,
            ), );
    }
}
