use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::designer::selection::Selected;

use self::{
    bounding_box_gizmos::{draw_bounding_boxes, BoundingBoxGizmos},
    debug_mode_state::{toggle_debug_mode, DebugModeState},
    window_fps_display::display_fps,
};

mod bounding_box_gizmos;
mod debug_mode_state;
mod window_fps_display;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new().run_if(in_state(DebugModeState::On)))
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .init_state::<DebugModeState>()
            .init_gizmo_group::<BoundingBoxGizmos>()
            .add_systems(Update, toggle_debug_mode)
            .add_systems(
                Update,
                draw_bounding_boxes.run_if(in_state(DebugModeState::On)),
            )
            .add_systems(Update, display_fps);
    }
}
