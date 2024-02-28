use crate::{
    get_cursor, get_cursor_mut,
    simulation::{
        chip::{Chip, ChipExtents},
        events::{SpawnChipEvent, SpawnIOPinEvent},
        io_pin::{
            BoardBinaryIOHandleBar, BoardBinaryIOHandleBarExtents, BoardBinaryInput,
            BoardBinaryInputPin, BoardBinaryInputSwitch, BoardBinaryOutput, BoardBinaryOutputPin,
        },
        signal_state::SignalState,
        wire::Wire,
    },
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::*;

use super::{
    bounding_box::BoundingBox,
    chip::{ChipInputPin, ChipOutputPin},
    draw_layer::DrawLayer,
    render_settings::CircuitBoardRenderingSettings,
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

pub fn spawn_chip_at_cursor(
    key_input: Res<ButtonInput<KeyCode>>,
    mut ev_writer: EventWriter<SpawnChipEvent>,
    q_cursor: Query<&Transform, With<Cursor>>,
) {
    let cursor_transform = get_cursor!(q_cursor);

    if key_input.just_pressed(KeyCode::KeyF) {
        ev_writer.send(SpawnChipEvent {
            chip_name: "and".to_string(),
            position: cursor_transform.translation.xy(),
        });
    } else if key_input.just_pressed(KeyCode::KeyG) {
        ev_writer.send(SpawnChipEvent {
            chip_name: "or".to_string(),
            position: cursor_transform.translation.xy(),
        });
    } else if key_input.just_pressed(KeyCode::KeyH) {
        ev_writer.send(SpawnChipEvent {
            chip_name: "xor".to_string(),
            position: cursor_transform.translation.xy(),
        });
    } else if key_input.just_pressed(KeyCode::KeyJ) {
        ev_writer.send(SpawnChipEvent {
            chip_name: "not".to_string(),
            position: cursor_transform.translation.xy(),
        });
    }
}

pub fn spawn_io_pin_at_cursor(
    input: Res<ButtonInput<KeyCode>>,
    mut ev_writer: EventWriter<SpawnIOPinEvent>,
    q_cursor: Query<&Transform, With<Cursor>>,
) {
    let cursor_transform = get_cursor!(q_cursor);

    let spawn_input_pin = if input.just_pressed(KeyCode::KeyI) {
        true
    } else if input.just_pressed(KeyCode::KeyO) {
        false
    } else {
        return;
    };

    ev_writer.send(SpawnIOPinEvent {
        is_input: spawn_input_pin,
        position: cursor_transform.translation.xy(),
    });
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
        if bbox.point_in_bbox(cursor_transform.translation.truncate()) {
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

    //TODO: fix board binary io handlebars
    if input.just_pressed(MouseButton::Left) && cursor.state == CursorState::Idle {
        for (draggable_entity, bbox, mut draggable_entity_transform) in
            q_draggable_entities.iter_mut()
        {
            if bbox.point_in_bbox(cursor_transform.translation.truncate()) {
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

// #[allow(clippy::type_complexity)]
// pub fn drag_chip(
//     input: Res<ButtonInput<MouseButton>>,
//     mut q_chips: Query<(&GlobalTransform, &mut Transform, &ChipExtents, Entity), With<Chip>>,
//     mut q_cursor: Query<(&mut Cursor, &Transform), (With<Cursor>, Without<Chip>)>,
// ) {
//     let (mut cursor, cursor_transform) = get_cursor_mut!(q_cursor);

//     if let CursorState::DraggingChip(dragged_chip_entity) = cursor.state {
//         if input.pressed(MouseButton::Left) {
//             for (_, mut chip_transform, _, chip_entity) in q_chips.iter_mut() {
//                 if chip_entity != dragged_chip_entity {
//                     continue;
//                 }

//                 chip_transform.translation = cursor_transform
//                     .translation
//                     .xy()
//                     .extend(chip_transform.translation.z);
//             }

//             return;
//         }

//         if input.just_released(MouseButton::Left) {
//             cursor.state = CursorState::Idle;
//         }
//     }

//     if input.just_pressed(MouseButton::Left) && cursor.state == CursorState::Idle {
//         for (chip_global_transform, _, chip_extents, chip_entity) in q_chips.iter_mut() {
//             let chip_position: Vec2 = Vec2::new(
//                 chip_global_transform.translation().x,
//                 chip_global_transform.translation().y,
//             );

//             let cursor_on_chip: bool = cursor_transform.translation.x
//                 >= chip_position.x - (chip_extents.0.x / 2.0)
//                 && cursor_transform.translation.x <= chip_position.x + (chip_extents.0.x / 2.0)
//                 && cursor_transform.translation.y >= chip_position.y - (chip_extents.0.y / 2.0)
//                 && cursor_transform.translation.y <= chip_position.y + (chip_extents.0.y / 2.0);

//             if !cursor_on_chip {
//                 continue;
//             }

//             //window.set_cursor_icon(CursorIcon::Grab);
//             cursor.state = CursorState::DraggingChip(chip_entity);
//             return;
//         }
//     }
// }

// #[allow(clippy::type_complexity)]
// pub fn drag_board_binary_io(
//     input: Res<ButtonInput<MouseButton>>,
//     mut q_cursor: Query<
//         (&mut Cursor, &Transform),
//         (
//             With<Cursor>,
//             Without<BoardBinaryInput>,
//             Without<BoardBinaryOutput>,
//         ),
//     >,
//     mut q_bbio_handle_bars: Query<
//         (
//             &GlobalTransform,
//             &BoardBinaryIOHandleBarExtents,
//             Entity,
//             &Parent,
//         ),
//         With<BoardBinaryIOHandleBar>,
//     >,
//     mut q_bbio: Query<&mut Transform, Or<(With<BoardBinaryInput>, With<BoardBinaryOutput>)>>,
//     render_settings: Res<CircuitBoardRenderingSettings>,
// ) {
//     let (mut cursor, cursor_transform) = get_cursor_mut!(q_cursor);

//     if let CursorState::DraggingBBIO(dragged_entity) = cursor.state {
//         if input.pressed(MouseButton::Left) {
//             for (_, _, handle_bar_entity, bbio_entity) in q_bbio_handle_bars.iter() {
//                 if handle_bar_entity != dragged_entity {
//                     continue;
//                 }

//                 let mut bbio_transform = q_bbio.get_mut(bbio_entity.get())
//                         .expect("BoardBinaryInputHandleBar has no parent BoardBinaryInput or BoardBinaryOutput.");

//                 bbio_transform.translation = cursor_transform
//                     .translation
//                     .xy()
//                     .extend(bbio_transform.translation.z);
//             }

//             return;
//         }

//         if input.just_released(MouseButton::Left) {
//             cursor.state = CursorState::Idle;
//         }
//     }

//     if input.just_pressed(MouseButton::Left) && cursor.state == CursorState::Idle {
//         for (handle_bar_global_transform, handle_bar_extents, handle_bar_entity, _) in
//             q_bbio_handle_bars.iter_mut()
//         {
//             let handle_bar_position: Vec2 = Vec2::new(
//                 handle_bar_global_transform.translation().x,
//                 handle_bar_global_transform.translation().y,
//             );

//             let cursor_on_handle_bar: bool = cursor_transform.translation.x
//                     >= handle_bar_position.x - (handle_bar_extents.0.x / 2.0)
//                         + render_settings.binary_io_pin_radius // Workaround weil die handle bar unterm switch liegt
//                     && cursor_transform.translation.x <= handle_bar_position.x + (handle_bar_extents.0.x / 2.0)
//                     && cursor_transform.translation.y >= handle_bar_position.y - (handle_bar_extents.0.y / 2.0)
//                     && cursor_transform.translation.y <= handle_bar_position.y + (handle_bar_extents.0.y / 2.0);

//             if !cursor_on_handle_bar {
//                 continue;
//             }

//             //window.set_cursor_icon(CursorIcon::Grab);
//             cursor.state = CursorState::DraggingBBIO(handle_bar_entity);
//             return;
//         }
//     }
// }

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn drag_wire(
    input: Res<ButtonInput<MouseButton>>,
    q_chip_output_pins: Query<(), With<ChipOutputPin>>,
    q_wire_src_pins: Query<
        (&GlobalTransform, Entity),
        (
            Or<(With<ChipOutputPin>, With<BoardBinaryInputPin>)>,
            Without<Camera>,
        ),
    >,
    q_input_pins: Query<(&GlobalTransform, Entity), (With<ChipInputPin>, Without<Camera>)>,
    q_board_output_pins: Query<
        (&GlobalTransform, Entity),
        (With<BoardBinaryOutputPin>, Without<Camera>),
    >,
    mut q_wires: Query<(&mut Path, &GlobalTransform, &mut Wire)>,
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
                    transform: Transform::from_xyz(0.0, 0.0, DrawLayer::Wire.get_z()),
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

        for (pin_transform, pin_entity) in q_wire_src_pins.iter() {
            let pin_radius = if q_chip_output_pins.get(pin_entity).is_ok() {
                render_settings.chip_pin_radius
            } else {
                render_settings.binary_io_pin_radius
            };

            if cursor_transform
                .translation
                .truncate()
                .distance(pin_transform.translation().truncate())
                > pin_radius
            {
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
        if let Ok(wire_components) = q_wires.get_mut(wire_entity) {
            let (mut path, output_pin_transform, mut wire) = wire_components;

            //TODO: nicht jedes mal abfrage? aber juckt glaube nicht
            if wire.dest_pin.is_some() {
                wire.dest_pin = None;
            }

            if input.pressed(MouseButton::Left) {
                *path = ShapePath::build_as(&shapes::Line(
                    Vec2::ZERO,
                    cursor_transform.translation.truncate()
                        - output_pin_transform.translation().truncate(),
                ));
            } else if input.just_released(MouseButton::Left) {
                let hovered_chip_pin = || {
                    q_input_pins.iter().find(|pin| {
                        cursor_transform
                            .translation
                            .truncate()
                            .distance(pin.0.translation().truncate())
                            <= render_settings.chip_pin_radius
                    })
                };

                let hovered_board_pin = || {
                    q_board_output_pins.iter().find(|pin| {
                        cursor_transform
                            .translation
                            .truncate()
                            .distance(pin.0.translation().truncate())
                            <= render_settings.binary_io_pin_radius
                    })
                };

                let hovered_pin = hovered_chip_pin().or_else(hovered_board_pin);

                if let Some(hovered_pin) = hovered_pin {
                    // connect wire to pin
                    wire.dest_pin = Some(hovered_pin.1);
                    *path = ShapePath::build_as(&shapes::Line(
                        Vec2::ZERO,
                        hovered_pin.0.translation().truncate()
                            - output_pin_transform.translation().truncate(),
                    ));
                    cursor.state = CursorState::Idle;
                    return;
                }

                // delete wire if dragged on nothing
                commands.entity(wire_entity).despawn();
                cursor.state = CursorState::Idle;
            }
        }
    }
}

pub fn toggle_board_input_pin(
    input: Res<ButtonInput<MouseButton>>,
    q_inputs: Query<&Children, With<BoardBinaryInput>>,
    q_input_switches: Query<(&GlobalTransform, &Parent), With<BoardBinaryInputSwitch>>,
    mut q_input_pins: Query<(&mut BoardBinaryInputPin, &mut SignalState)>,
    render_settings: Res<CircuitBoardRenderingSettings>,
    q_cursor: Query<&Transform, With<Cursor>>,
) {
    let cursor_transform = get_cursor!(q_cursor);

    if input.just_pressed(MouseButton::Left) {
        for (switch_transform, parent) in q_input_switches.iter() {
            if cursor_transform
                .translation
                .truncate()
                .distance(switch_transform.translation().truncate())
                > render_settings.binary_io_pin_radius
            {
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
