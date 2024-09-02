pub mod board_binary_io;
pub mod board_entity;
pub mod bounding_box;
pub mod chip;
pub mod clock;
pub mod cursor;
pub mod designer_state;
pub mod macros;
pub mod pin;
pub mod render_settings;
pub mod selection;
pub mod signal_state;
pub mod wire;

use bevy::prelude::*;
use bevy::transform::TransformSystem;
use board_binary_io::{BoardBinaryInput, BoardBinaryOutput};
use board_entity::{
    manage_additional_spawn_tasks, update_board_entity_position, BoardEntityModel,
    BoardEntityViewKind, Position,
};
use chip::{BuiltinChip, Chip};
use clock::{spawn_clock, tick_clocks, Clock};
use moonshine_save::load::load_from_file_on_event;
use moonshine_save::save::save_default;
use moonshine_view::RegisterView;
use pin::{update_previous_signal_states, PinModelCollection};
use selection::{release_drag, update_dragged_entities_position};
use signal_state::SignalState;
use wire::Wire;

use crate::events::events::{LoadEvent, SaveEvent};
use crate::simulation::simulation::update_signals;
use crate::ui::cursor_captured::IsCursorCaptured;

use self::board_binary_io::spawn_board_binary_input;
use self::board_binary_io::spawn_board_binary_output;
use self::board_binary_io::toggle_board_input_switch;
use self::board_binary_io::update_board_binary_displays;
use self::bounding_box::update_bounding_boxes;
use self::chip::spawn_chip;
use self::cursor::highlight_hovered_pin;
use self::cursor::spawn_cursor;
use self::cursor::update_cursor;
use self::designer_state::DesignerState;
use self::render_settings::init_render_settings;
use self::selection::delete_selected;
use self::selection::highlight_selected;
use self::selection::select_single;
use self::selection::spawn_selection_box;
use self::selection::start_drag;
use self::selection::update_selection_box;
use self::signal_state::update_signal_colors;
use self::wire::drag_wire;
use self::wire::update_wires;

pub struct DesignerPlugin;

impl Plugin for DesignerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DesignerState>()
            .register_type::<BoardEntityModel>()
            .register_type::<Position>()
            .register_type::<BoardBinaryInput>()
            .register_type::<BoardBinaryOutput>()
            .register_type::<Chip>()
            .register_type::<BuiltinChip>()
            .register_type::<PinModelCollection>()
            .register_type::<BoardBinaryOutput>()
            .register_type::<Wire>()
            .register_type::<SignalState>()
            .register_type::<Clock>()
            .register_view::<BoardEntityViewKind, BoardBinaryInput>()
            .register_view::<BoardEntityViewKind, BoardBinaryOutput>()
            .register_view::<BoardEntityViewKind, Chip>()
            .register_view::<BoardEntityViewKind, Wire>()
            .register_view::<BoardEntityViewKind, Clock>()
            .add_systems(Startup, spawn_cursor)
            .add_systems(PreUpdate, update_cursor)
            .add_systems(PreUpdate, tick_clocks)
            .add_systems(
                PreUpdate,
                (
                    save_default().into_file_on_event::<SaveEvent>(),
                    load_from_file_on_event::<LoadEvent>(),
                ),
            )
            .add_systems(
                Update,
                drag_wire.run_if(resource_equals(IsCursorCaptured(false))),
            )
            .add_systems(
                Update,
                (
                    spawn_selection_box,
                    (select_single, start_drag).chain().after(drag_wire),
                    delete_selected,
                )
                    .after(drag_wire)
                    .run_if(resource_equals(IsCursorCaptured(false))),
            )
            .add_systems(Update, release_drag)
            .add_systems(Update, update_selection_box)
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
            .add_systems(Update, spawn_clock.pipe(manage_additional_spawn_tasks))
            .add_systems(Update, update_signal_colors.after(update_signals))
            .add_systems(Update, toggle_board_input_switch)
            .add_systems(
                Update,
                update_board_binary_displays
                    .after(toggle_board_input_switch)
                    .after(update_signals),
            )
            .add_systems(Update, update_board_entity_position)
            .add_systems(Update, update_wires)
            .add_systems(PostUpdate, update_dragged_entities_position)
            .add_systems(PostUpdate, highlight_selected)
            .add_systems(PostUpdate, update_previous_signal_states)
            .add_systems(
                PostUpdate,
                update_bounding_boxes.after(TransformSystem::TransformPropagate),
            );

        init_render_settings(app);
    }
}
