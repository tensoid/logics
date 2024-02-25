pub mod board_entity;
pub mod chip;
pub mod cursor;
pub mod draw_layer;
pub mod events;
pub mod expressions;
pub mod input;
pub mod io_pin;
pub mod render_settings;
pub mod signal_state;
pub mod simulation;
pub mod utils;
pub mod wire;

use bevy::prelude::*;

use signal_state::update_signal_colors;
use simulation::tick_simulation;

use chip::spawn_chip_event;
use io_pin::spawn_io_pin_event;
use wire::update_wires;

use cursor::delete_board_entity;
use cursor::drag_board_binary_io;
use cursor::drag_chip;
use cursor::drag_wire;
use cursor::spawn_chip_at_cursor;
use cursor::spawn_io_pin_at_cursor;
use cursor::toggle_board_input_pin;
use cursor::update_cursor;

use self::events::register_events;
use self::input::handle_keybindings;
use self::input::register_keybindings;
use self::render_settings::register_render_settings;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(tick_simulation)
            .add_system(update_signal_colors.after(tick_simulation))
            .add_system(spawn_chip_at_cursor)
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
            .add_system(update_cursor) //TODO: run before anything
            .add_system(handle_keybindings);

        register_events(app);
        register_keybindings(app);
        register_render_settings(app);
    }
}
