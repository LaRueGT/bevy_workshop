use bevy::prelude::*;
use bevy::utils::Duration;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(AsteroidSpawnTimer(Timer::from_seconds(
            1.0,
            TimerMode::Once,
        )))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                sprite_movement,
                ship_movement_input,
                bullet_firing,
                spawn_asteroids,
                asteroid_bullet_collision,
                despawn_entities_outside_of_screen,
                confine_player_movement,
                player_hit,
            )
        )
        .run();
}
////////////////
//components
//
/////////////////
#[derive(Component)]
struct Player;

#[derive(Component)]
struct SpriteMovement {
    direction: Vec3,
    speed: f32,
}

#[derive(Component)]
struct Bullet;

//Deref? DerefMut?
#[derive(Component, Deref, DerefMut)]
struct CooldownTimer(Timer);

#[derive(Component)]
struct Asteroid;

#[derive(Resource, Deref, DerefMut)]
struct AsteroidSpawnTimer(Timer);

#[derive(Component)]
struct BallCollider(f32);

//////////////////////
//systems
//
//////////////////////
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //spawn Camera
    commands.spawn(Camera2dBundle::default());
    // Spawn the spaceship
    commands.spawn(  (
        Player,
        SpriteBundle {
            texture: asset_server.load("spaceship.png"),
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

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&SpriteMovement, &mut Transform)>) {
    for (movement, mut transform) in &mut sprite_position {
        transform.translation +=
            movement.direction.normalize_or_zero() * movement.speed * time.delta_seconds();
    }
}

fn ship_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut SpriteMovement, With<Player>>,
    )
{
    let mut sprite_movement = player.single_mut();

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

fn bullet_firing(
    mut cmd: Commands,
    mut player: Query<(&Transform, &mut CooldownTimer), With<Player>>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    let (translation, mut timer) = player.single_mut();
    timer.tick(time.delta());
    let position = translation.translation + Vec3::new(0.0, 30.0, 0.0);

    if keyboard_input.pressed(KeyCode::Space) && timer.finished() {
        cmd.spawn((
            Bullet,
            SpriteBundle {
                texture: asset_server.load("bullet.png"),
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
        cmd.spawn(AudioBundle {
            source: sound_effect,
            settings: PlaybackSettings::DESPAWN,
        });
        timer.reset();
    }
}

fn spawn_asteroids(
    mut cmd: Commands,
    window: Query<&Window>,
    time: Res<Time>,
    mut timer: ResMut<AsteroidSpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    timer.tick(time.delta());

    if timer.finished() {
        let mut rng = rand::thread_rng();

        let window = window.single();
        let half_width = window.resolution.width() / 2.0;
        let half_height = window.resolution.height() / 2.0;

        // Spawn an asteroid
        cmd.spawn((
            Asteroid,
            SpriteBundle {
                texture: asset_server.load("asteroid.png"),
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
        timer.set_duration(Duration::from_millis(rng.gen_range(500..3000)));
        timer.reset();
    }
}

fn asteroid_bullet_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform, &BallCollider), With<Bullet>>,
    asteroids: Query<(Entity, &Transform, &BallCollider), With<Asteroid>>,
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
            }
        }
    }
}

fn player_hit(
    mut commands: Commands,
    mut player: Query<(Entity, &Transform, &BallCollider), With<Player>>,
    asteroids: Query<(Entity, &Transform, &BallCollider), With<Asteroid>>,
    asset_server: Res<AssetServer>,
) {
    let (player_entity, player_transform, player_collider) = player.single_mut();
    for (asteroid_entity, asteroid_transform, asteroid_collider) in &mut asteroids.iter(){
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
            //causes panic, trying to fix
            commands.entity(player_entity).despawn();
        }
    }
}

fn despawn_entities_outside_of_screen(
    mut cmd: Commands,
    window: Query<&Window>,
    query: Query<(Entity, &Transform), Or<(With<Asteroid>, With<Bullet>)>>,
) {
    let window = window.single();
    // Add buffer so that objects aren't despawned until they are completely off the screen
    let half_height = window.resolution.height() / 2.0 + 100.0;

    for (entity, transform) in &mut query.iter() {
        if transform.translation.y < -half_height || transform.translation.y > half_height {
            cmd.entity(entity).despawn();
        }
    }
}

fn confine_player_movement (
    mut player: Query<&mut Transform, With<Player>>,
    window: Query<&Window>,
){
    let mut transform = player.single_mut();
    let mut translation = transform.translation;
    let window = window.single();

    let player_radius = 24.0;
    let x_min = -window.width()/ 2.0 + player_radius;
    let x_max = window.width()/2.0 - player_radius;
    let y_min = -window.height()/2.0 + player_radius;
    let y_max = window.height()/2.0 - player_radius;

    //clamp the X position
    if translation.x < x_min {
        translation.x = x_min;
    } else if translation.x > x_max{
        translation.x = x_max;
    }
    //clamp the y position
    if translation.y < y_min {
        translation.y = y_min;
    } else if translation . y > y_max {
        translation.y = y_max;
    }
    transform.translation = translation;
}
