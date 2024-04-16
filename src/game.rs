pub mod player;
pub mod asteroid;
pub mod collectible;
pub mod score;

//children
use crate::events::*;
use crate::game::asteroid::*;
use crate::game::collectible::*;
use crate::game::player::*;
use crate::game::score::*;

//extern
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
            .add_plugins((
                AsteroidPlugin,
                PlayerPlugin,
                ScorePlugin,
                CollectiblePlugin, ))
            .add_systems(
                Update,
                (
                    sprite_movement,
                ));
    }
}

#[derive(Component)]
pub struct SpriteMovement {
    direction: Vec3,
    speed: f32,
}

#[derive(Component)]
pub struct BallCollider(f32);

pub fn sprite_movement(
    time: Res<Time>,
    mut sprite_position: Query<(&SpriteMovement, &mut Transform)>,
) {
    for (movement, mut transform) in &mut sprite_position {
        transform.translation +=
            movement.direction.normalize_or_zero() * movement.speed * time.delta_seconds();
    }
}

