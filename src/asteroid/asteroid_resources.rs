use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct AsteroidSpawnTimer(Timer);

#[derive(Default)]
#[derive(Resource)]
pub struct AsteroidSpawnCount {
    value: u32,
}

#[derive(Resource)]
pub struct AsteroidSpawnLimit {
    pub value: u32,
}

impl Default for AsteroidSpawnLimit {
    fn default() -> AsteroidSpawnLimit {
        AsteroidSpawnLimit { value: 10 }
    }
}