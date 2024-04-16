use bevy::prelude::*;

pub mod collectible_components;
pub mod collectible_systems;
pub mod collectible_resources;

use collectible_resources::*;
use collectible_systems::*;
use crate::AppState;
use crate::game::SimulationState;

pub struct CollectiblePlugin;

impl Plugin for CollectiblePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CollectibleSpawnTimer(Timer::from_seconds(
            5.0,
            TimerMode::Once,
        )));
        app.init_resource::<CollectibleSpawnCount>();
        app.init_resource::<CollectibleSpawnLimit>();
        app.add_systems(Update, (
            spawn_collectibles, )
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running)), );
    }
}