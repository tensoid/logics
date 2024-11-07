pub mod bounding_box;
pub mod copy_paste;
pub mod cursor;
pub mod designer_assets;
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

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::transform::TransformSystem;
use copy_paste::CopyPastePlugin;
use cursor::CursorPlugin;
use designer_assets::load_assets;
use devices::DevicePlugin;
use file_dialog::{handle_load_request, handle_save_request, ActiveSaveFile};
use moonshine_save::load::load_from_file_on_event;
use moonshine_save::save::save_default;
use pin::PinPlugin;
use selection::SelectionPlugin;
use wire::WirePlugin;

use crate::events::events::{LoadEvent, LoadRequestEvent, SaveEvent, SaveRequestEvent};
use crate::simulation::simulation::update_signals;

use self::bounding_box::update_bounding_boxes;
use self::designer_state::DesignerState;
use self::render_settings::init_render_settings;
use self::signal_state::update_signal_colors;

pub struct DesignerPlugins;

impl PluginGroup for DesignerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(DevicePlugin)
            .add(CopyPastePlugin)
            .add(WirePlugin)
            .add(SelectionPlugin)
            .add(CursorPlugin)
            .add(PinPlugin)
            .add(DesignerPlugin)
    }
}

pub struct DesignerPlugin;

impl Plugin for DesignerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DesignerState>()
            .init_resource::<ActiveSaveFile>()
            .add_systems(PreStartup, load_assets)
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
            .add_systems(Update, update_signal_colors.after(update_signals)) //TODO: observers?
            .add_systems(
                Update,
                handle_save_request.run_if(on_event::<SaveRequestEvent>()),
            )
            .add_systems(
                Update,
                handle_load_request.run_if(on_event::<LoadRequestEvent>()),
            )
            .add_systems(
                PostUpdate,
                update_bounding_boxes.after(TransformSystem::TransformPropagate),
            );

        init_render_settings(app);
    }
}
