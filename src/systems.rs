use bevy::prelude::*;
use bevy::app::AppExit;
//internal
use crate::events::*;

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