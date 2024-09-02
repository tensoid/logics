use bevy::prelude::*;
use simulation::{evaluate_builtin_chips, reset_input_pins};

use crate::designer::{
    board_binary_io::toggle_board_input_switch, clock::tick_clocks, wire::update_wires,
};

use self::simulation::update_signals;

pub mod simulation;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (evaluate_builtin_chips, reset_input_pins, update_signals)
                .chain()
                .after(update_wires)
                .after(tick_clocks)
                .after(toggle_board_input_switch),
        );
    }
}
