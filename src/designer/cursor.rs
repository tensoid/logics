use crate::{
    designer::{
        chip::Chip,
        io_pin::{BoardBinaryInput, BoardBinaryInputPin, BoardBinaryOutputPin},
        signal_state::SignalState,
        wire::Wire,
    },
    get_cursor, get_cursor_mut,
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::*;

use super::{
    bounding_box::BoundingBox,
    chip::{ChipInputPin, ChipOutputPin},
    io_pin::BoardBinaryInputSwitch,
    render_settings::{self, CircuitBoardRenderingSettings},
};

#[derive(PartialEq)]
pub enum CursorState {
    Idle,
    Dragging,
    DraggingWire(Entity),
}

#[derive(Component)]
pub struct Cursor {
    pub state: CursorState,
}

#[derive(Bundle, Default)]
pub struct CursorBundle {
    cursor: Cursor,
    spatial: SpatialBundle,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            state: CursorState::Idle,
        }
    }
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

pub fn delete_board_entity(
    q_cursor: Query<(&Cursor, &Transform)>,
    q_deletable_entities: Query<(Entity, &BoundingBox), Without<Cursor>>,
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
) {
    let (cursor, cursor_transform) = get_cursor!(q_cursor);

    if cursor.state != CursorState::Idle {
        return;
    }

    if !input.just_pressed(MouseButton::Right) {
        return;
    }

    for (deletable_entity, bbox) in q_deletable_entities.iter() {
        if bbox.point_in_bbox(cursor_transform.translation.truncate()) && bbox.interactable {
            commands.entity(deletable_entity).despawn_recursive();
            return;
        }
    }
}

pub fn drag_board_entity(
    input: Res<ButtonInput<MouseButton>>,
    mut q_cursor: Query<(Entity, &mut Cursor, &Transform, Option<&Children>)>,
    mut q_draggable_entities: Query<(Entity, &BoundingBox, &mut Transform), Without<Cursor>>,
    mut commands: Commands,
) {
    let (cursor_entity, mut cursor, cursor_transform, cursor_children) = get_cursor_mut!(q_cursor);

    if cursor.state == CursorState::Dragging && input.just_released(MouseButton::Left) {
        cursor.state = CursorState::Idle;

        for &cursor_child_entity in cursor_children.iter().flat_map(|c| c.iter()) {
            let (_, _, mut child_transform) =
                q_draggable_entities.get_mut(cursor_child_entity).unwrap();
            child_transform.translation =
                cursor_transform.translation + child_transform.translation;
        }

        commands.entity(cursor_entity).clear_children();
    }

    if input.just_pressed(MouseButton::Left) && cursor.state == CursorState::Idle {
        for (draggable_entity, bbox, mut draggable_entity_transform) in
            q_draggable_entities.iter_mut()
        {
            if bbox.point_in_bbox(cursor_transform.translation.truncate()) && bbox.interactable {
                cursor.state = CursorState::Dragging;
                commands.entity(cursor_entity).add_child(draggable_entity);
                let position_diff =
                    draggable_entity_transform.translation - cursor_transform.translation;
                draggable_entity_transform.translation = position_diff;
                return;
            }
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn drag_wire(
    input: Res<ButtonInput<MouseButton>>,
    q_wire_src_pins: Query<
        (&BoundingBox, Entity),
        (
            Or<(With<ChipOutputPin>, With<BoardBinaryInputPin>)>,
            Without<Camera>,
        ),
    >,
    q_wire_dest_pins: Query<
        (&BoundingBox, Entity),
        (
            Or<(With<ChipInputPin>, With<BoardBinaryOutputPin>)>,
            Without<Camera>,
        ),
    >,
    mut q_wires: Query<&mut Wire>,
    mut q_cursor: Query<(&mut Cursor, &Transform), With<Cursor>>,
    mut commands: Commands,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    let (mut cursor, cursor_transform) = get_cursor_mut!(q_cursor);

    if input.just_pressed(MouseButton::Left) && cursor.state == CursorState::Idle {
        let wire_bundle = (
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Line(Vec2::ZERO, Vec2::ZERO)),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.005),
                    ..default()
                },
                ..default()
            },
            Stroke::new(
                render_settings.signal_low_color,
                render_settings.wire_line_width,
            ),
            SignalState::Low,
        );

        for (bbox, pin_entity) in q_wire_src_pins.iter() {
            if !bbox.point_in_bbox(cursor_transform.translation.truncate()) {
                continue;
            }

            // cursor is on pin
            let wire = commands
                .spawn((
                    wire_bundle,
                    Wire {
                        src_pin: Some(pin_entity),
                        dest_pin: None,
                    },
                ))
                .id();
            cursor.state = CursorState::DraggingWire(wire);
            return;
        }
    }

    if let CursorState::DraggingWire(wire_entity) = cursor.state {
        if let Ok(mut wire) = q_wires.get_mut(wire_entity) {
            if input.just_released(MouseButton::Left) {
                for (bbox, pin_entity) in q_wire_dest_pins.iter() {
                    if bbox.point_in_bbox(cursor_transform.translation.truncate()) {
                        // connect wire to pin
                        wire.dest_pin = Some(pin_entity);
                        cursor.state = CursorState::Idle;
                        return;
                    }
                }

                // delete wire if dragged on nothing
                commands.entity(wire_entity).despawn();
                cursor.state = CursorState::Idle;
            }
        }
    }
}

pub fn toggle_board_input_switch(
    input: Res<ButtonInput<MouseButton>>,
    q_inputs: Query<&Children, With<BoardBinaryInput>>,
    q_input_switches: Query<(&Parent, &BoundingBox), With<BoardBinaryInputSwitch>>,
    mut q_input_pins: Query<(&mut BoardBinaryInputPin, &mut SignalState)>,
    q_cursor: Query<&Transform, With<Cursor>>,
) {
    let cursor_transform = get_cursor!(q_cursor);

    if input.just_pressed(MouseButton::Left) {
        for (parent, bbox) in q_input_switches.iter() {
            if !bbox.point_in_bbox(cursor_transform.translation.truncate()) {
                continue;
            }

            //TODO: find a way to make this easier (Child -> Parent -> Children -> Specific Children)
            let parent_children = q_inputs
                .get(parent.get())
                .expect("BoardBinaryInputSwitch has no BoardBinaryInput parent.");

            let board_binary_input_pin_entity = parent_children
                .iter()
                .find(|c| q_input_pins.get(**c).is_ok())
                .expect("BoardBinaryInput has no BoardBinaryInputPin child.");

            let (_, mut board_binary_input_pin_state) = q_input_pins
                .get_mut(*board_binary_input_pin_entity)
                .expect("BoardBinaryInput has no BoardBinaryInputPin child.");

            board_binary_input_pin_state.toggle();
            break;
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
