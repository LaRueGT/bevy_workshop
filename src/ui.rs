//extern
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            main_menu,
        ));
    }
}

pub fn main_menu() {
    print!("--In Main Menu--");
}