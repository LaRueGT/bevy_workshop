//extern
use bevy::prelude::*;
//children
use crate::player::player_components::*;
use crate::player::player_resources::*;
use crate::{BallCollider, SpriteMovement};
//parents
use crate::asteroid::asteroid_components::*;
use crate::asteroid::asteroid_resources::*;
use crate::collectible::collectible_components::*;
use crate::collectible::collectible_resources::*;
use crate::score::score_resources::*;
use crate::events::*;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the spaceship
    commands.spawn((
        Player,
        SpriteBundle {
            texture: asset_server.load("sprites/spaceship.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                .with_scale(Vec3::splat(2.)),
            ..default()
        },
        SpriteMovement {
            direction: Vec3::splat(0.0),
            speed: 100.0,
        },
        CooldownTimer(Timer::from_seconds(0.2, TimerMode::Once)),
        BallCollider(24.0),
    ));
}

pub fn ship_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut SpriteMovement, With<Player>>,
) {
    if let Ok(mut sprite_movement) = player.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::KeyA) || keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            sprite_movement.direction.x = -1.0;
        } else if (keyboard_input.just_released(KeyCode::KeyA)
            || keyboard_input.just_released(KeyCode::ArrowLeft))
            && sprite_movement.direction.x < 0.0
        {
            sprite_movement.direction.x = 0.0;
        }

        if keyboard_input.just_pressed(KeyCode::KeyD) || keyboard_input.just_pressed(KeyCode::ArrowRight) {
            sprite_movement.direction.x = 1.0;
        } else if (keyboard_input.just_released(KeyCode::KeyD)
            || keyboard_input.just_released(KeyCode::ArrowRight))
            && sprite_movement.direction.x > 0.0
        {
            sprite_movement.direction.x = 0.0;
        }

        if keyboard_input.just_pressed(KeyCode::KeyW) || keyboard_input.just_pressed(KeyCode::ArrowUp) {
            sprite_movement.direction.y = 1.0;
        } else if (keyboard_input.just_released(KeyCode::KeyW)
            || keyboard_input.just_released(KeyCode::ArrowUp))
            && sprite_movement.direction.y > 0.0
        {
            sprite_movement.direction.y = 0.0;
        }

        if keyboard_input.just_pressed(KeyCode::KeyS) || keyboard_input.just_pressed(KeyCode::ArrowDown) {
            sprite_movement.direction.y = -1.0;
        } else if (keyboard_input.just_released(KeyCode::KeyS)
            || keyboard_input.just_released(KeyCode::ArrowDown))
            && sprite_movement.direction.y < 0.0
        {
            sprite_movement.direction.y = 0.0;
        }
    }
}

pub fn bullet_firing(
    mut commands: Commands,
    mut player: Query<(&Transform, &mut CooldownTimer), With<Player>>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((translation, mut timer)) = player.get_single_mut() {
        timer.tick(time.delta());
        let position = translation.translation + Vec3::new(0.0, 30.0, 0.0);

        if keyboard_input.pressed(KeyCode::Space) && timer.finished() {
            commands.spawn((
                Bullet,
                SpriteBundle {
                    texture: asset_server.load("sprites/bullet.png"),
                    transform: Transform::from_translation(position),
                    ..default()
                },
                SpriteMovement {
                    direction: Vec3::Y,
                    speed: 200.0,
                },
                BallCollider(2.0),
            ));
            let sound_effect = asset_server.load("audio/laserSmall_000.ogg");
            commands.spawn(AudioBundle {
                source: sound_effect,
                settings: PlaybackSettings::DESPAWN,
            });
            timer.reset();
        }
    }
}

pub fn player_hit(
    mut commands: Commands,
    mut player: Query<(Entity, &Transform, &BallCollider), With<Player>>,
    asteroids: Query<(Entity, &Transform, &BallCollider), With<Asteroid>>,
    asset_server: Res<AssetServer>,
    mut count: ResMut<AsteroidSpawnCount>,
    mut game_over_eventwriter: EventWriter<GameOver>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform, player_collider)) = player.get_single_mut() {
        for (asteroid_entity, asteroid_transform, asteroid_collider) in &mut asteroids.iter() {
            if asteroid_transform
                .translation
                .distance(player_transform.translation)
                < asteroid_collider.0 + player_collider.0
            {
                let sound_effect = asset_server.load("audio/explosionCrunch_004.ogg");
                commands.spawn(AudioBundle {
                    source: sound_effect,
                    settings: PlaybackSettings::DESPAWN,
                });
                commands.entity(asteroid_entity).despawn();
                count.value -= 1;
                commands.entity(player_entity).despawn();
                game_over_eventwriter.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn player_collect(
    mut commands: Commands,
    mut player: Query<(Entity, &Transform, &BallCollider), With<Player>>,
    collectibles: Query<(Entity, &Transform, &BallCollider), With<Collectible>>,
    asset_server: Res<AssetServer>,
    mut count: ResMut<CollectibleSpawnCount>,
    mut score: ResMut<Score>,
) {
    if let Ok((_player_entity, player_transform, player_collider)) = player.get_single_mut() {
        for (collectible_entity, collectible_transform, collectible_collider) in &mut collectibles.iter() {
            if collectible_transform
                .translation
                .distance(player_transform.translation)
                < collectible_collider.0 + player_collider.0
            {
                let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                commands.spawn(AudioBundle {
                    source: sound_effect,
                    settings: PlaybackSettings::DESPAWN,
                });
                commands.entity(collectible_entity).despawn();
                count.value -= 1;
                score.value += 1;
            }
        }
    }
}

pub fn confine_player_movement(
    mut player: Query<&mut Transform, With<Player>>,
    window: Query<&Window>,
) {
    if let Ok(mut transform) = player.get_single_mut() {
        let mut translation = transform.translation;
        let window = window.single();

        let player_radius = 24.0;
        let x_min = -window.width() / 2.0 + player_radius;
        let x_max = window.width() / 2.0 - player_radius;
        let y_min = -window.height() / 2.0 + player_radius;
        let y_max = window.height() / 2.0 - player_radius;

        //clamp the X position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        //clamp the y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }
        transform.translation = translation;
    }
}
