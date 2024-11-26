use bevy::prelude::*;
use simulation::{evaluate_builtin_chips, reset_input_pins};

use crate::designer::wire::update_wires;

use self::simulation::propagate_signals;

pub mod simulation;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (evaluate_builtin_chips, reset_input_pins, propagate_signals).chain(),
        );
    }
}
