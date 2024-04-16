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
        .add_systems(
            Startup,
            (
                setup_camera,
            ),
        )
        .add_plugins((
            UIPlugin,
            GamePlugin,
        ))
        .add_systems(
            Update,
            (
                exit_game,
                handle_game_over,
            ),
        )
        .run();
}
