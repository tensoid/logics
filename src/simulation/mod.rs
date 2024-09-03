use bevy::prelude::*;
use simulation::{evaluate_builtin_chips, reset_input_pins};


use self::simulation::update_signals;

pub mod simulation;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (evaluate_builtin_chips, reset_input_pins, update_signals).chain(),
        );
    }
}
