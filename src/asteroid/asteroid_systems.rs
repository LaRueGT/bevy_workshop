//std
use rand::Rng;
//extern
use bevy::prelude::*;
use bevy::utils::Duration;
//children
use crate::asteroid::asteroid_components::*;
use crate::asteroid::asteroid_resources::*;
//parents
use crate::{BallCollider, SpriteMovement};
use crate::player::player_components::*;
use crate::score::score_resources::*;

pub fn spawn_asteroids(
    mut commands: Commands,
    window: Query<&Window>,
    time: Res<Time>,
    mut timer: ResMut<AsteroidSpawnTimer>,
    asset_server: Res<AssetServer>,
    limit: Res<AsteroidSpawnLimit>,
    mut count: ResMut<AsteroidSpawnCount>,
) {
    timer.tick(time.delta());

    if timer.finished() && count.value < limit.value {
        let mut rng = rand::thread_rng();

        let window = window.single();
        let half_width = window.resolution.width() / 2.0;
        let half_height = window.resolution.height() / 2.0;

        // Spawn an asteroid
        commands.spawn((
            Asteroid,
            SpriteBundle {
                texture: asset_server.load("sprites/asteroid.png"),
                transform: Transform::from_translation(Vec3::new(
                    rng.gen_range(-half_width..half_width),
                    half_height + 100.0, // Add buffer so that objects don't appear on screen from thin air
                    0.0,
                ))
                    .with_scale(Vec3::splat(2.0)),
                ..default()
            },
            SpriteMovement {
                direction: Vec3::new(0.0, -1.0, 0.0),
                speed: 30.0,
            },
            BallCollider(24.0),
        ));
        count.value += 1;
        timer.set_duration(Duration::from_millis(rng.gen_range(500..3000)));
        timer.reset();
    }
}

pub fn asteroid_bullet_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform, &BallCollider), With<Bullet>>,
    asteroids: Query<(Entity, &Transform, &BallCollider), With<Asteroid>>,
    mut score: ResMut<Score>,
    mut count: ResMut<AsteroidSpawnCount>,
) {
    for (bullet_entity, bullet_transform, bullet_collider) in &mut bullets.iter() {
        for (asteroid_entity, asteroid_transform, asteroid_collider) in &mut asteroids.iter() {
            if bullet_transform
                .translation
                .distance(asteroid_transform.translation)
                < bullet_collider.0 + asteroid_collider.0
            {
                commands.entity(bullet_entity).despawn();
                commands.entity(asteroid_entity).despawn();
                score.value += 1;
                count.value -= 1;
            }
        }
    }
}

pub fn despawn_asteroids_outside_of_screen(
    mut commands: Commands,
    window: Query<&Window>,
    query: Query<(Entity, &Transform, Has<Asteroid>), Or<(With<Asteroid>, With<Bullet>)>>,
    mut count: ResMut<AsteroidSpawnCount>,
) {
    let window = window.single();
    let half_height = window.resolution.height() / 2.0 + 100.0;

    for (entity, transform, is_asteroid) in &mut query.iter() {
        if transform.translation.y < -half_height || transform.translation.y > half_height {
            if is_asteroid {
                count.value -= 1;
            }
            commands.entity(entity).despawn();
        }
    }
}

