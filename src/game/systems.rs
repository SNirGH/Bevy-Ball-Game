use bevy::prelude::*;

use super::SimulationState;

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if *simulation_state.get() == SimulationState::Running {
            next_state.set(SimulationState::Paused);
            println!("Simulation Paused");
        }
        if *simulation_state.get() == SimulationState::Paused {
            next_state.set(SimulationState::Running);
            println!("Simulation Running");
        }
    }
}
