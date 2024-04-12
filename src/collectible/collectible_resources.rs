use bevy::prelude::{Deref, DerefMut, Resource, Timer};

#[derive(Resource, Deref, DerefMut)]
pub struct CollectibleSpawnTimer(Timer);

#[derive(Default)]
#[derive(Resource)]
pub struct CollectibleSpawnCount {
    value: u32,
}

#[derive(Resource)]
pub struct CollectibleSpawnLimit {
    pub value: u32,
}

impl Default for CollectibleSpawnLimit {
    fn default() -> CollectibleSpawnLimit {
        CollectibleSpawnLimit { value: 5 }
    }
}
