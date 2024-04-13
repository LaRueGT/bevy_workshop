//module structure
pub mod events;
pub mod player;
pub mod asteroid;
pub mod collectible;
pub mod score;

//extern
use bevy::prelude::*;
use bevy::app::AppExit;
//children
use crate::events::*;
use crate::asteroid::*;
use crate::collectible::*;
use crate::player::*;
use crate::score::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_event::<GameOver>()
        .add_plugins((
            AsteroidPlugin,
            PlayerPlugin,
            ScorePlugin,
            CollectiblePlugin,))
        .add_systems(
            Startup, (
                setup_camera,
            ),)
        .add_systems(
            Update,
            (
                sprite_movement,
                exit_game,
                handle_game_over,
            ),
        )
        .run();
}

#[derive(Component)]
pub struct SpriteMovement {
    direction: Vec3,
    speed: f32,
}

#[derive(Component)]
pub struct BallCollider(f32);

fn setup_camera(mut commands: Commands) {
    //spawn Camera
    commands.spawn(Camera2dBundle::default());
}

pub fn sprite_movement(
    time: Res<Time>,
    mut sprite_position: Query<(&SpriteMovement, &mut Transform)>,
) {
    for (movement, mut transform) in &mut sprite_position {
        transform.translation +=
            movement.direction.normalize_or_zero() * movement.speed * time.delta_seconds();
    }
}

fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_eventwriter: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_eventwriter.send(AppExit);
    }
}

fn handle_game_over(
    mut game_over_eventreader: EventReader<GameOver>
) {
    for event in game_over_eventreader.read() {
        println!("Your final score is: {}", event.score.to_string());
    }
}