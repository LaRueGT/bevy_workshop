use bevy::prelude::*;
use bevy::app::AppExit;

//internal
use crate::events::*;
use crate::AppState;
use crate::game::SimulationState;

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_eventwriter: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_eventwriter.send(AppExit);
    }
}

pub fn handle_game_over(
    mut game_over_eventreader: EventReader<GameOver>
) {
    for event in game_over_eventreader.read() {
        println!("Your final score is: {}", event.score.to_string());
    }
}

pub fn setup_camera(mut commands: Commands) {
    //spawn Camera
    commands.spawn(Camera2dBundle::default());
}

pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if *app_state.get() != AppState::Game {
            next_app_state.set(AppState::Game);
            println!("Entered AppState::Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if *app_state.get() != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            println!("Entered AppState::MainMenu");
        }
    }
}
