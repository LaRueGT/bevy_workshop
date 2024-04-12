mod events;

mod player {
    pub mod player_components;
    pub mod player_systems;
    pub mod player_resources;
}

mod asteroid {
    pub mod asteroid_components;
    pub mod asteroid_systems;
    pub mod asteroid_resources;
}

mod collectible {
    pub mod collectible_components;
    pub mod collectible_systems;
    pub mod collectible_resources;
}

mod score {
    pub mod score_components;
    pub mod score_systems;
    pub mod score_resources;
}

use bevy::prelude::*;
use bevy::app::AppExit;

use crate::events::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(AsteroidSpawnTimer(Timer::from_seconds(
            1.0,
            TimerMode::Once,
        )))
        .init_resource::<Score>()
        .init_resource::<AsteroidSpawnCount>()
        .init_resource::<AsteroidSpawnLimit>()
        .insert_resource(CollectibleSpawnTimer(Timer::from_seconds(
            5.0,
            TimerMode::Once,
        )))
        .init_resource::<CollectibleSpawnCount>()
        .init_resource::<CollectibleSpawnLimit>()
        .add_event::<GameOver>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                sprite_movement,
                ship_movement_input,
                bullet_firing,
                spawn_asteroids,
                spawn_collectibles,
                asteroid_bullet_collision,
                despawn_entities_outside_of_screen,
                confine_player_movement,
                player_hit,
                player_collect,
                update_score,
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