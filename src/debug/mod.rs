use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use debug_mode_settings::DebugModeSettings;
use entity_ids::draw_entity_ids;

use self::{
    bounding_box_gizmos::{draw_bounding_boxes, BoundingBoxGizmos},
    debug_mode_state::{toggle_debug_mode, DebugModeState},
};

mod bounding_box_gizmos;
mod debug_mode_settings;
mod debug_mode_state;
mod entity_ids;
mod window_fps_display;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new().run_if(in_state(DebugModeState::On)))
            .add_plugins(
                ResourceInspectorPlugin::<DebugModeSettings>::new()
                    .run_if(in_state(DebugModeState::On)),
            )
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .init_resource::<DebugModeSettings>()
            .register_type::<DebugModeSettings>()
            .init_state::<DebugModeState>()
            .init_gizmo_group::<BoundingBoxGizmos>()
            .add_systems(
                Update,
                draw_entity_ids.run_if(
                    resource_changed::<DebugModeSettings>.or(state_changed::<DebugModeState>),
                ),
            )
            .add_systems(Update, toggle_debug_mode)
            .add_systems(
                Update,
                draw_bounding_boxes.run_if(should_draw_bounding_boxes),
            );
    }
}

fn should_draw_bounding_boxes(
    debug_mode_state: Res<State<DebugModeState>>,
    debug_mode_settings: Res<DebugModeSettings>,
) -> bool {
    debug_mode_state.eq(&DebugModeState::On) && debug_mode_settings.draw_bounding_boxes
}
