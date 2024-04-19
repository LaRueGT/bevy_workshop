use bevy::prelude::*;

#[derive(Default)]
#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

#[derive(Default)]
#[derive(Resource)]
pub struct FinalScore {
    pub value: u32,
}