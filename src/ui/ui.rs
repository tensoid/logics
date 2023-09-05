use super::circuit_board::*;
use super::cursor::*;
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_chip_at_cursor)
            .add_system(spawn_io_pin_at_cursor)
            .add_system(spawn_chip_event)
            .add_system(drag_board_binary_io)
            .add_system(toggle_board_input_pin.before(drag_board_binary_io))
            .add_system(spawn_io_pin_event)
            .add_system(drag_chip)
            .add_system(drag_wire.before(drag_board_binary_io).before(drag_chip))
            // runs in post update because it requires that all despawning of dest pins has been completed to update the wires
            .add_system(update_wires.in_base_set(CoreSet::PostUpdate))
            .add_system(delete_board_entity)
            .add_system(update_cursor); //TODO: run before anything
    }
}
