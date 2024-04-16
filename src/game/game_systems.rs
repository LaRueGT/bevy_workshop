use bevy::prelude::*;
use crate::game::SimulationState;

pub fn toggle_simulation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        if *simulation_state.get() == SimulationState::Running {
            next_simulation_state.set(SimulationState::Paused);
            println!("Simulation Paused")
        } else if *simulation_state.get() == SimulationState::Paused {
            next_simulation_state.set(SimulationState::Running);
            println!("Simulation Resumed")
        }
    }
}