pub mod board_entity;
pub mod bounding_box;
pub mod chip;
pub mod cursor;
pub mod draw_layer;
pub mod io_pin;
pub mod macros;
pub mod render_settings;
pub mod signal_state;
pub mod wire;

use bevy::prelude::*;
use bevy::transform::TransformSystem;

use crate::simulation::simulation::tick_simulation;

use self::bounding_box::update_bounding_boxes;
use self::chip::spawn_chip_event;
use self::cursor::delete_board_entity;
use self::cursor::drag_board_entity;
use self::cursor::drag_wire;
use self::cursor::spawn_chip_at_cursor;
use self::cursor::spawn_cursor;
use self::cursor::spawn_io_pin_at_cursor;
use self::cursor::toggle_board_input_pin;
use self::cursor::update_cursor;
use self::io_pin::spawn_io_pin_event;
use self::render_settings::init_render_settings;
use self::signal_state::update_signal_colors;
use self::wire::update_wires;

pub struct DesignerPlugin;

impl Plugin for DesignerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cursor)
            .add_systems(PreUpdate, update_cursor)
            .add_systems(Update, spawn_chip_at_cursor)
            .add_systems(Update, spawn_io_pin_at_cursor)
            .add_systems(Update, spawn_chip_event)
            .add_systems(Update, spawn_io_pin_event)
            .add_systems(Update, drag_board_entity)
            .add_systems(Update, update_signal_colors.after(tick_simulation))
            .add_systems(Update, toggle_board_input_pin.before(drag_board_entity))
            .add_systems(Update, drag_wire.before(drag_board_entity))
            .add_systems(Update, delete_board_entity)
            // runs in post update because it requires that all despawning of dest pins has been completed to update the wires
            .add_systems(PostUpdate, update_wires)
            .add_systems(
                PostUpdate,
                update_bounding_boxes.after(TransformSystem::TransformPropagate),
            );

        init_render_settings(app);
    }
}
