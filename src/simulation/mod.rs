use bevy::prelude::*;
use simulation::evaluate_builtin_chips;

use crate::designer::wire::update_wires;

use self::simulation::update_signals;

pub mod simulation;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        //TODO: currently running in post update because wires need to be deleted first if src or dest pin has been deleted
        app.add_systems(
            PostUpdate,
            (evaluate_builtin_chips, update_signals.after(update_wires)).chain(),
        );
    }
}
