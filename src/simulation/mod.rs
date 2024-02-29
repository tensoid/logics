use bevy::prelude::*;

use self::simulation::tick_simulation;

pub mod expressions;
pub mod simulation;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_simulation);
    }
}
