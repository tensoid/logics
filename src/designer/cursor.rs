use crate::{
    designer::{
        chip::Chip,
        board_binary_io::{BoardBinaryInputPin, BoardBinaryOutputPin},
    },
    get_cursor, get_cursor_mut,
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::*;

use super::{
    bounding_box::BoundingBox,
    chip::{ChipInputPin, ChipOutputPin},
    render_settings::CircuitBoardRenderingSettings,
};

#[derive(PartialEq, Default)]
pub enum CursorState {
    #[default]
    Idle,
    Dragging,
    DraggingWire(Entity),
    Selecting,
}

#[derive(Component, Default)]
pub struct Cursor {
    pub state: CursorState,
}

#[derive(Bundle, Default)]
pub struct CursorBundle {
    cursor: Cursor,
    spatial: SpatialBundle,
}

pub fn screen_to_world_space(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    position: Vec2,
) -> Vec2 {
    camera
        .viewport_to_world(camera_transform, position)
        .map(|ray| ray.origin.truncate())
        .unwrap()
}

pub fn spawn_cursor(mut commands: Commands) {
    commands.spawn(CursorBundle::default());
}

pub fn update_cursor(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), Without<Chip>>,
    mut q_cursor: Query<&mut Transform, With<Cursor>>,
) {
    let mut cursor_transform = get_cursor_mut!(q_cursor);

    if let Ok(window) = q_window.get_single() {
        if q_camera.iter().count() > 1 {
            panic!("More than one camera in the scene.");
        }
        for (camera, camera_transform) in q_camera.iter() {
            if let Some(cursor_screen_pos) = window.cursor_position() {
                cursor_transform.translation =
                    screen_to_world_space(camera, camera_transform, cursor_screen_pos).extend(0.0);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn highlight_hovered_pin(
    q_cursor: Query<&Transform, With<Cursor>>,
    mut q_pins: Query<
        (&BoundingBox, &mut Fill),
        Or<(
            With<ChipInputPin>,
            With<ChipOutputPin>,
            With<BoardBinaryInputPin>,
            With<BoardBinaryOutputPin>,
        )>,
    >,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    let cursor_position = get_cursor!(q_cursor).translation.truncate();

    for (bbox, mut fill) in q_pins.iter_mut() {
        if bbox.point_in_bbox(cursor_position) {
            *fill = Fill::color(render_settings.hovered_pin_color)
        } else {
            *fill = Fill::color(render_settings.pin_color)
        }
    }
}
