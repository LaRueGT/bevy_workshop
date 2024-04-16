use bevy::prelude::*;

pub mod asteroid_components;
pub mod asteroid_systems;
pub mod asteroid_resources;

use crate::game::asteroid::asteroid_resources::*;
use crate::game::asteroid::asteroid_systems::*;
use crate::AppState;
use crate::game::SimulationState;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsteroidSpawnTimer(Timer::from_seconds(
            1.0,
            TimerMode::Once, )));
        app.init_resource::<AsteroidSpawnCount>();
        app.init_resource::<AsteroidSpawnLimit>();
        app.add_systems(Update, (
            spawn_asteroids,
            asteroid_bullet_collision,
            despawn_asteroids_outside_of_screen,
        )
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running)),
        );
    }
}
