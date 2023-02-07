use super::circuit_board::*;
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_chip_at_cursor)
            .add_system(spawn_chip_event)
            .add_system(drag_chip)
            .add_system(drag_wire)
            .add_system(update_wires)
            .add_system(delete_chip);
        //.add_system(update_cursor_state);
    }
}
