use bevy::prelude::*;

pub mod player_components;
pub mod player_systems;
pub mod player_resources;

use crate::player::player_systems::*;

pub struct  PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup,
            spawn_player
        );
        app.add_systems(Update, (
            ship_movement_input,
            bullet_firing,
            confine_player_movement,
            player_hit,
            player_collect,
        ),);
    }
}