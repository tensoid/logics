pub mod board_entity;
pub mod bounding_box;
pub mod chip;
pub mod cursor;
pub mod designer_state;
pub mod io_pin;
pub mod macros;
pub mod render_settings;
mod save_load;
pub mod selection;
pub mod signal_state;
pub mod wire;

use bevy::prelude::*;
use bevy::transform::TransformSystem;

use crate::simulation::simulation::tick_simulation;
use crate::ui::cursor_captured::IsCursorCaptured;

use self::board_entity::manage_additional_spawn_tasks;
use self::bounding_box::update_bounding_boxes;
use self::chip::spawn_chip;
use self::cursor::highlight_hovered_pin;
use self::cursor::spawn_cursor;
use self::cursor::update_cursor;
use self::designer_state::DesignerState;
use self::io_pin::spawn_board_binary_input;
use self::io_pin::spawn_board_binary_output;
use self::io_pin::toggle_board_input_switch;
use self::io_pin::update_board_binary_displays;
use self::render_settings::init_render_settings;
use self::selection::clear_selection;
use self::selection::delete_selected;
use self::selection::drag_selected;
use self::selection::highlight_selected;
use self::selection::select_single;
use self::selection::spawn_selection_box;
use self::selection::stop_dragging;
use self::selection::update_selection_box;
use self::signal_state::update_signal_colors;
use self::wire::drag_wire;
use self::wire::update_wires;

pub struct DesignerPlugin;

impl Plugin for DesignerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DesignerState>()
            .add_systems(Startup, spawn_cursor)
            .add_systems(PreUpdate, update_cursor)
            .add_systems(
                Update,
                drag_wire.run_if(resource_equals(IsCursorCaptured(false))),
            )
            .add_systems(Update, stop_dragging)
            .add_systems(
                Update,
                (
                    spawn_selection_box,
                    (clear_selection, select_single, drag_selected)
                        .chain()
                        .after(drag_wire),
                    delete_selected,
                )
                    .after(drag_wire)
                    .run_if(resource_equals(IsCursorCaptured(false))),
            )
            .add_systems(Update, update_selection_box)
            .add_systems(Update, highlight_selected)
            .add_systems(Update, highlight_hovered_pin)
            .add_systems(Update, spawn_chip.pipe(manage_additional_spawn_tasks))
            .add_systems(
                Update,
                spawn_board_binary_input.pipe(manage_additional_spawn_tasks),
            )
            .add_systems(
                Update,
                spawn_board_binary_output.pipe(manage_additional_spawn_tasks),
            )
            .add_systems(Update, update_signal_colors.after(tick_simulation))
            .add_systems(Update, toggle_board_input_switch)
            .add_systems(
                Update,
                update_board_binary_displays
                    .after(toggle_board_input_switch)
                    .after(tick_simulation),
            )
            // runs in post update because it requires that all despawning of dest pins has been completed to update the wires
            .add_systems(PostUpdate, update_wires)
            .add_systems(
                PostUpdate,
                update_bounding_boxes.after(TransformSystem::TransformPropagate),
            );

        init_render_settings(app);
    }
}
