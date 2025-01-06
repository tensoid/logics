pub mod bounding_box;
pub mod copy_paste;
pub mod cursor;
pub mod designer_state;
pub mod devices;
pub mod macros;
pub mod model;
pub mod pin;
pub mod position;
pub mod render_settings;
pub mod rotation;
pub mod save_management;
pub mod selection;
pub mod signal;
pub mod wire;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::transform::TransformSystem;
use copy_paste::CopyPastePlugin;
use cursor::CursorPlugin;
use devices::DevicePlugin;
use model::{ModelId, ModelRegistry};
use pin::PinPlugin;
use save_management::SaveManagementPlugin;
use selection::SelectionPlugin;
use signal::SignalState;
use wire::WirePlugin;

use self::bounding_box::update_bounding_boxes;
use self::designer_state::DesignerState;
use self::render_settings::init_render_settings;

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
            .add(SaveManagementPlugin)
            .add(DesignerPlugin)
    }
}

pub struct DesignerPlugin;

impl Plugin for DesignerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DesignerState>()
            .init_resource::<ModelRegistry>()
            .register_type::<SignalState>()
            .register_type::<ModelId>()
            .register_type::<ModelRegistry>()
            .add_systems(
                PostUpdate,
                update_bounding_boxes.after(TransformSystem::TransformPropagate),
            );

        init_render_settings(app);
    }
}
