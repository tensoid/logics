use bevy::{math::bounding::BoundingVolume, prelude::*};

use super::{bounding_box::BoundingBox, events::ToggleDebugModeEvent};

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum DebugModeState {
    #[default]
    Off,
    On,
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct BoundingBoxGizmos;

pub fn init_debug_settings(app: &mut App) {
    app.init_state::<DebugModeState>()
        .init_gizmo_group::<BoundingBoxGizmos>();
}

pub fn toggle_debug_mode(
    debug_mode_state: Res<State<DebugModeState>>,
    mut debug_mode_next_state: ResMut<NextState<DebugModeState>>,
    mut toggle_debug_mode_ev: EventReader<ToggleDebugModeEvent>,
) {
    for _ in toggle_debug_mode_ev.read() {
        let next_state = match debug_mode_state.get() {
            DebugModeState::Off => DebugModeState::On,
            DebugModeState::On => DebugModeState::Off,
        };
        debug_mode_next_state.set(next_state);
    }
}

pub fn draw_bounding_boxes(
    mut bbox_gizmos: Gizmos<BoundingBoxGizmos>,
    q_bboxes: Query<&BoundingBox>,
) {
    for bbox in q_bboxes.iter() {
        bbox_gizmos.rect_2d(
            bbox.aabb.center(),
            0.0,
            bbox.aabb.half_size() * 2.0,
            Color::RED,
        );
    }
}
