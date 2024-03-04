pub mod board_entity;
pub mod bounding_box;
pub mod chip;
pub mod cursor;
pub mod io_pin;
pub mod macros;
pub mod render_settings;
pub mod signal_state;
pub mod wire;

use bevy::prelude::*;
use bevy::transform::TransformSystem;

use crate::simulation::simulation::tick_simulation;
use crate::ui::chip_selector::ChipSelectorState;

use self::bounding_box::update_bounding_boxes;
use self::chip::spawn_chip_event;
use self::cursor::delete_board_entity;
use self::cursor::drag_board_entity;
use self::cursor::drag_wire;
use self::cursor::highlight_hovered_pin;
use self::cursor::spawn_cursor;
use self::cursor::toggle_board_input_switch;
use self::cursor::update_cursor;
use self::io_pin::spawn_board_binary_input;
use self::io_pin::spawn_board_binary_output;
use self::io_pin::update_board_binary_displays;
use self::render_settings::init_render_settings;
use self::signal_state::update_signal_colors;
use self::wire::update_wires;

pub struct DesignerPlugin;

impl Plugin for DesignerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cursor)
            .add_systems(PreUpdate, update_cursor)
            .add_systems(Update, highlight_hovered_pin)
            .add_systems(Update, spawn_chip_event)
            .add_systems(Update, spawn_board_binary_input)
            .add_systems(Update, spawn_board_binary_output)
            .add_systems(
                Update,
                drag_board_entity.run_if(in_state(ChipSelectorState::Closed)),
            )
            .add_systems(Update, update_signal_colors.after(tick_simulation))
            .add_systems(Update, toggle_board_input_switch)
            .add_systems(
                Update,
                update_board_binary_displays
                    .after(toggle_board_input_switch)
                    .after(tick_simulation),
            )
            .add_systems(
                Update,
                drag_wire
                    .before(drag_board_entity)
                    .run_if(in_state(ChipSelectorState::Closed)),
            )
            .add_systems(
                Update,
                delete_board_entity.run_if(in_state(ChipSelectorState::Closed)),
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
