use bevy::prelude::*;
use simulation::{apply_signals, evaluate_builtin_chips};

use self::simulation::propagate_signals;

pub mod simulation;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (evaluate_builtin_chips, propagate_signals, apply_signals).chain(),
        );
    }
}
