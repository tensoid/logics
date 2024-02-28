pub mod board_entity;
pub mod bounding_box;
pub mod chip;
pub mod cursor;
pub mod debug;
pub mod draw_layer;
pub mod events;
pub mod expressions;
pub mod input;
pub mod io_pin;
pub mod macros;
pub mod render_settings;
pub mod signal_state;
pub mod simulation;
pub mod wire;

use bevy::prelude::*;

use bevy::transform::TransformSystem;
use signal_state::update_signal_colors;
use simulation::tick_simulation;

use chip::spawn_chip_event;
use io_pin::spawn_io_pin_event;
use wire::update_wires;

use cursor::delete_board_entity;
// use cursor::drag_board_binary_io;
// use cursor::drag_chip;
use cursor::drag_wire;
use cursor::spawn_chip_at_cursor;
use cursor::spawn_io_pin_at_cursor;
use cursor::toggle_board_input_pin;
use cursor::update_cursor;

use self::bounding_box::update_bounding_boxes;
use self::cursor::drag_board_entity;
use self::cursor::spawn_cursor;
use self::debug::draw_bounding_boxes;
use self::debug::init_debug_settings;
use self::debug::toggle_debug_mode;
use self::debug::DebugModeState;
use self::events::register_events;
use self::input::handle_keybindings;
use self::input::register_keybindings;
use self::render_settings::init_render_settings;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cursor)
            .add_systems(Update, tick_simulation)
            .add_systems(Update, update_signal_colors.after(tick_simulation))
            .add_systems(Update, spawn_chip_at_cursor)
            .add_systems(Update, spawn_io_pin_at_cursor)
            .add_systems(Update, spawn_chip_event)
            .add_systems(Update, spawn_io_pin_event)
            .add_systems(Update, drag_board_entity)
            .add_systems(Update, toggle_board_input_pin.before(drag_board_entity))
            .add_systems(Update, drag_wire.before(drag_board_entity))
            .add_systems(Update, delete_board_entity)
            .add_systems(Update, update_cursor) //TODO: run before anything
            .add_systems(Update, handle_keybindings)
            .add_systems(Update, toggle_debug_mode)
            .add_systems(
                Update,
                draw_bounding_boxes.run_if(in_state(DebugModeState::On)),
            )
            .add_systems(
                PostUpdate,
                update_bounding_boxes.after(TransformSystem::TransformPropagate),
            )
            // runs in post update because it requires that all despawning of dest pins has been completed to update the wires
            .add_systems(PostUpdate, update_wires);

        register_events(app);
        register_keybindings(app);
        init_render_settings(app);
        init_debug_settings(app);
    }
}
