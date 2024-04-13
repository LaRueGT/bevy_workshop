use bevy::prelude::*;
use bevy::utils::Duration;

pub mod asteroid_components;
pub mod asteroid_systems;
pub mod asteroid_resources;

use crate::asteroid::asteroid_resources::*;
use crate::asteroid::asteroid_systems::*;

pub struct AsteroidPlugin{}

impl Plugin for AsteroidPlugin{
    fn build(&self, app: &mut App){
        app.insert_resource(AsteroidSpawnTimer(Timer::from_seconds(
            1.0,
            TimerMode::Once, )));
        app.init_resource::<AsteroidSpawnCount>();
        app.init_resource::<AsteroidSpawnLimit>();
        app.add_systems( Update, (
                spawn_asteroids,
                asteroid_bullet_collision,
                despawn_asteroids_outside_of_screen,
        ),);
    }
}
