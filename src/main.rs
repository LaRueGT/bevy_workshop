pub mod events;
pub mod game;
pub mod ui;
pub mod systems;

//extern
use bevy::prelude::*;
//children
use crate::systems::*;
use crate::game::*;
use crate::ui::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .init_state::<AppState>()
        .add_plugins((
            UIPlugin,
            GamePlugin,
        ))
        .add_systems(
            Startup,
            (
                setup_camera,
            ),
        )
        .add_systems(
            Update,
            (
                transition_to_game_state,
                transition_to_main_menu_state,
                exit_game,
                handle_game_over,
            ),
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}