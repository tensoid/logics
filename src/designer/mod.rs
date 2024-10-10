pub mod assets;
pub mod bounding_box;
pub mod copy_paste;
pub mod cursor;
pub mod designer_state;
pub mod devices;
pub mod file_dialog;
pub mod macros;
pub mod pin;
pub mod position;
pub mod render_settings;
pub mod selection;
pub mod signal_state;
pub mod wire;

use assets::load_assets;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::transform::TransformSystem;
use copy_paste::{
    copy_devices, copy_wires, paste_devices, paste_wires, DeviceClipboard, WireClipboard,
};
use devices::binary_io::{toggle_binary_switch, update_board_binary_displays};
use devices::device::update_device_positions;
use devices::DevicePlugin;
use file_dialog::{handle_load_request, handle_save_request, ActiveSaveFile};
use moonshine_save::load::load_from_file_on_event;
use moonshine_save::save::save_default;
use pin::commit_signal_updates;
use selection::{release_drag, select_all, update_dragged_entities_position};

use crate::events::events::{
    CopyEvent, LoadEvent, LoadRequestEvent, PasteEvent, SaveEvent, SaveRequestEvent, SelectAllEvent,
};
use crate::simulation::simulation::update_signals;
use crate::ui::cursor_captured::IsCursorCaptured;

use self::bounding_box::update_bounding_boxes;
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

pub struct DesignerPlugins;

impl PluginGroup for DesignerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(DevicePlugin)
            .add(DesignerPlugin)
    }
}

pub struct DesignerPlugin;

impl Plugin for DesignerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DesignerState>()
            .init_resource::<ActiveSaveFile>()
            .add_systems(PreStartup, load_assets)
            .add_systems(Startup, spawn_cursor)
            .add_systems(PreUpdate, update_cursor)
            .add_systems(
                PreUpdate,
                (
                    // Needs additional on_event condition because of the use of has_event in the moonshine_save crate.
                    // has_event doesnt consume the event and because of that it executes the pipeline multiple times per event which causes a crash.
                    // This might be fixed in the latest version of moonshine_save which is not yet published on crates io.
                    save_default()
                        .into_file_on_event::<SaveEvent>()
                        .run_if(on_event::<SaveEvent>()),
                    load_from_file_on_event::<LoadEvent>().run_if(on_event::<LoadEvent>()),
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
                    delete_selected, //TODO: remove cursor captured condition for delete
                )
                    .after(drag_wire)
                    .run_if(resource_equals(IsCursorCaptured(false))),
            )
            .add_systems(Update, select_all.run_if(on_event::<SelectAllEvent>()))
            .add_systems(Update, release_drag)
            .add_systems(Update, update_selection_box)
            .add_systems(Update, highlight_hovered_pin)
            .add_systems(Update, update_signal_colors.after(update_signals)) //TODO: observers?
            .add_systems(Update, toggle_binary_switch)
            .add_systems(
                Update,
                update_board_binary_displays
                    .after(toggle_binary_switch) //TODO: observers?
                    .after(update_signals),
            )
            .add_systems(Update, update_device_positions)
            .add_systems(Update, update_wires)
            .add_systems(
                Update,
                handle_save_request.run_if(on_event::<SaveRequestEvent>()),
            )
            .add_systems(
                Update,
                handle_load_request.run_if(on_event::<LoadRequestEvent>()),
            )
            .add_systems(PostUpdate, update_dragged_entities_position)
            .add_systems(PostUpdate, highlight_selected) //TODO: observers?
            .add_systems(PostUpdate, commit_signal_updates) //TODO: observers?
            .add_systems(
                PostUpdate,
                update_bounding_boxes.after(TransformSystem::TransformPropagate),
            );

        // copy / paste
        app.init_resource::<DeviceClipboard>();
        app.init_resource::<WireClipboard>();
        app.add_systems(
            Update,
            (copy_devices, copy_wires).run_if(on_event::<CopyEvent>()),
        )
        .add_systems(
            Update,
            (paste_devices.pipe(paste_wires)).run_if(on_event::<PasteEvent>()),
        );

        init_render_settings(app);
    }
}
