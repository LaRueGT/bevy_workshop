mod ui_systems;
mod ui_components;
mod ui_styles;

//extern
use bevy::prelude::*;
use crate::AppState;
use crate::ui::ui_systems::ui_interactions::*;
use crate::ui::ui_systems::ui_layout::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
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
    }
}

