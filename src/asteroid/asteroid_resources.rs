use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct AsteroidSpawnTimer(pub Timer);

#[derive(Default)]
#[derive(Resource)]
pub struct AsteroidSpawnCount {
    pub value: u32,
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