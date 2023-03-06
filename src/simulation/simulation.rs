use bevy::prelude::*;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(tick_simulation);
    }
}

fn tick_simulation() {}
