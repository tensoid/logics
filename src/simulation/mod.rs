use bevy::prelude::*;
use simulation::{evaluate_builtin_chips, handle_floating_pins};

use crate::designer::wire::update_wires;

use self::simulation::update_signals;

pub mod simulation;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        //TODO: currently running in post update because wires need to be deleted first if src or dest pin has been deleted
        app.add_systems(
            Update,
            (evaluate_builtin_chips, handle_floating_pins, update_signals)
                .chain()
                .after(update_wires),
        );
    }
}
