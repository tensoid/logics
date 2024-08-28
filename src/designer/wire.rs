use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{get_cursor, get_cursor_mut};

use super::{
    board_binary_io::{BoardBinaryInputPin, BoardBinaryOutputPin},
    bounding_box::BoundingBox,
    chip::{ChipInputPin, ChipOutputPin},
    cursor::{Cursor, CursorState},
    render_settings::CircuitBoardRenderingSettings,
    signal_state::SignalState,
};

#[derive(Component)]
pub struct Wire {
    pub src_pin: Option<Entity>,
    pub dest_pin: Option<Entity>,
}

#[derive(Bundle)]
pub struct WireBundle {
    wire: Wire,
    shape_bundle: ShapeBundle,
    stroke: Stroke,
    signal_state: SignalState,
}

impl WireBundle {
    pub fn new(render_settings: &CircuitBoardRenderingSettings, wire: Wire) -> Self {
        Self {
            wire,
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Line(Vec2::ZERO, Vec2::ZERO)),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.005),
                    ..default()
                },
                ..default()
            },
            stroke: Stroke::new(
                render_settings.signal_low_color,
                render_settings.wire_line_width,
            ),
            signal_state: SignalState::Low,
        }
    }
}

/**
 * Updates the wires location to always stay connected to its source and destination pins.
 * If the source or destination pin was deleted or the wire is just not connected this also deletes the wire entirely.
 */
//TODO: Optimisation potential with only updating necessary wires.
//TODO: maybe split into delete_dangling_wires and update_wires
//TODO: clean up the indents
#[allow(clippy::type_complexity)]
pub fn update_wires(
    mut q_wires: Query<(&mut Wire, &mut Path, &GlobalTransform, Entity)>,
    q_dest_pins: Query<&GlobalTransform, Or<(With<ChipInputPin>, With<BoardBinaryOutputPin>)>>,
    q_src_pins: Query<&GlobalTransform, Or<(With<ChipOutputPin>, With<BoardBinaryInputPin>)>>,
    q_cursor: Query<(&Cursor, &Transform)>,
    mut commands: Commands,
) {
    let (cursor, cursor_transform) = get_cursor!(q_cursor);

    for (wire, mut wire_path, _, wire_entity) in q_wires.iter_mut() {
        let Some(wire_src_pin_entity) = wire.src_pin else {
            commands.entity(wire_entity).despawn();
            continue;
        };

        if let Some(wire_dest_pin_entity) = wire.dest_pin {
            if let (Ok(wire_src_pin_transform), Ok(wire_dest_pin_transform)) = (
                q_src_pins.get(wire_src_pin_entity),
                q_dest_pins.get(wire_dest_pin_entity),
            ) {
                let new_wire = shapes::Line(
                    wire_src_pin_transform.translation().truncate(),
                    wire_dest_pin_transform.translation().truncate(),
                );

                *wire_path = ShapePath::build_as(&new_wire);
            } else {
                commands.entity(wire_entity).despawn();
                continue;
            }
        } else if let CursorState::DraggingWire(dragged_wire) = cursor.state {
            //TODO: move this to drag_wire
            if dragged_wire.eq(&wire_entity) {
                if let Ok(wire_src_pin_transform) = q_src_pins.get(wire_src_pin_entity) {
                    let new_wire = shapes::Line(
                        wire_src_pin_transform.translation().truncate(),
                        cursor_transform.translation.truncate(),
                    );

                    *wire_path = ShapePath::build_as(&new_wire);
                }
            } else {
                commands.entity(wire_entity).despawn();
            }
        } else {
            commands.entity(wire_entity).despawn();
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn drag_wire(
    input: Res<ButtonInput<MouseButton>>,
    q_wire_src_pins: Query<
        (&BoundingBox, Entity),
        Or<(With<ChipOutputPin>, With<BoardBinaryInputPin>)>,
    >,
    q_wire_dest_pins: Query<
        (&BoundingBox, Entity),
        Or<(With<ChipInputPin>, With<BoardBinaryOutputPin>)>,
    >,
    mut q_wires: Query<&mut Wire>,
    mut q_cursor: Query<(&mut Cursor, &Transform), With<Cursor>>,
    mut commands: Commands,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    let (mut cursor, cursor_transform) = get_cursor_mut!(q_cursor);

    if input.just_pressed(MouseButton::Left) && cursor.state == CursorState::Idle {
        for (bbox, pin_entity) in q_wire_src_pins.iter() {
            if !bbox.point_in_bbox(cursor_transform.translation.truncate()) {
                continue;
            }

            // cursor is on pin
            let wire = commands
                .spawn(WireBundle::new(
                    render_settings.as_ref(),
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
