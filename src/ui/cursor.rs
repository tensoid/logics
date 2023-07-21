use crate::simulation::{
    chip::{Chip, ChipExtents, SpawnChipEvent},
    pin::{
        BoardBinaryIOHandleBar, BoardBinaryIOHandleBarExtents, BoardBinaryInput,
        BoardBinaryInputPin, BoardBinaryInputSwitch, BoardBinaryOutput, BoardBinaryOutputPin,
        ChipInputPin, ChipOutputPin, SpawnIOPinEvent,
    },
    pin_state::PinState,
    wire::Wire,
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::*;

use super::{circuit_board::CircuitBoardRenderingSettings, utils::screen_to_world_space};

#[derive(PartialEq)]
pub enum CursorState {
    Idle,
    DraggingChip(Entity), //TODO: put IsBeingDragged component on entity instead or is selected marker component
    DraggingWire(Entity),
    DraggingBBIO(Entity),
}

#[derive(Resource)]
pub struct Cursor {
    pub state: CursorState,
    pub pressed: bool,
    pub dragged_left_before_release: bool,
    pub world_pos: Option<Vec2>,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            state: CursorState::Idle,
            world_pos: None,
            pressed: false,
            dragged_left_before_release: false,
        }
    }
}

//TODO: instead of setting position when dragging set parent, this needs cursor to be an entity

pub fn update_cursor(
    //input: Res<Input<MouseButton>>,
    //mut mouse_motion_ev: EventReader<MouseMotion>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), Without<Chip>>,
    mut cursor: ResMut<Cursor>,
) {
    if let Ok(window) = q_window.get_single() {
        if q_camera.iter().count() > 1 {
            panic!("More than one camera in the scene.");
        }
        for (camera, camera_transform) in q_camera.iter() {
            if let Some(cursor_screen_pos) = window.cursor_position() {
                cursor.world_pos = Some(screen_to_world_space(
                    camera,
                    camera_transform,
                    cursor_screen_pos,
                ));
            }
        }
    }
    //TODO: check if drag or click
}

pub fn spawn_chip_at_cursor(
    key_input: Res<Input<KeyCode>>,
    mut ev_writer: EventWriter<SpawnChipEvent>,
    cursor: Res<Cursor>,
) {
    if key_input.just_pressed(KeyCode::F) {
        if let Some(cursor_world_pos) = cursor.world_pos {
            ev_writer.send(SpawnChipEvent {
                chip_name: "and".to_string(),
                position: cursor_world_pos,
            });
        }
    } else if key_input.just_pressed(KeyCode::G) {
        if let Some(cursor_world_pos) = cursor.world_pos {
            ev_writer.send(SpawnChipEvent {
                chip_name: "or".to_string(),
                position: cursor_world_pos,
            });
        }
    } else if key_input.just_pressed(KeyCode::H) {
        if let Some(cursor_world_pos) = cursor.world_pos {
            ev_writer.send(SpawnChipEvent {
                chip_name: "xor".to_string(),
                position: cursor_world_pos,
            });
        }
    } else if key_input.just_pressed(KeyCode::J) {
        if let Some(cursor_world_pos) = cursor.world_pos {
            ev_writer.send(SpawnChipEvent {
                chip_name: "not".to_string(),
                position: cursor_world_pos,
            });
        }
    }
}

pub fn spawn_io_pin_at_cursor(
    input: Res<Input<KeyCode>>,
    mut ev_writer: EventWriter<SpawnIOPinEvent>,
    cursor: Res<Cursor>,
) {
    let spawn_input_pin = if input.just_pressed(KeyCode::I) {
        true
    } else if input.just_pressed(KeyCode::O) {
        false
    } else {
        return;
    };

    if let Some(cursor_world_pos) = cursor.world_pos {
        ev_writer.send(SpawnIOPinEvent {
            is_input: spawn_input_pin,
            position: cursor_world_pos,
        });
    }
}

pub fn delete_chip(
    input: Res<Input<KeyCode>>,
    q_chips: Query<(Entity, &GlobalTransform, &ChipExtents, &Children), With<Chip>>,
    mut commands: Commands,
    mut q_wires: Query<(&mut Wire, &mut Path), With<Wire>>,
    cursor: Res<Cursor>,
) {
    if cursor.state != CursorState::Idle {
        return;
    }

    if let Some(cursor_world_pos) = cursor.world_pos {
        if input.just_pressed(KeyCode::D) {
            for (chip_entity, chip_transform, chip_extents, chip_children) in q_chips.iter() {
                let chip_position: Vec2 = Vec2::new(
                    chip_transform.translation().x,
                    chip_transform.translation().y,
                );

                let cursor_on_chip: bool = cursor_world_pos.x
                    >= chip_position.x - (chip_extents.0.x / 2.0)
                    && cursor_world_pos.x <= chip_position.x + (chip_extents.0.x / 2.0)
                    && cursor_world_pos.y >= chip_position.y - (chip_extents.0.y / 2.0)
                    && cursor_world_pos.y <= chip_position.y + (chip_extents.0.y / 2.0);

                if !cursor_on_chip {
                    continue;
                }

                // Despawn wires connected to chip
                for (mut wire, mut wire_path) in q_wires.iter_mut() {
                    if let Some(dest_pin) = wire.dest_pin {
                        for &chip_child in chip_children.iter() {
                            if dest_pin == chip_child {
                                wire.dest_pin = None;
                                *wire_path =
                                    ShapePath::build_as(&shapes::Line(Vec2::ZERO, Vec2::ZERO));
                            }
                        }
                    }
                }

                // Despawn chip and children
                commands.entity(chip_entity).despawn_recursive();
            }
        }
    }
}

//TODO: make one function for dragging everything
pub fn drag_chip(
    input: Res<Input<MouseButton>>,
    mut q_chips: Query<(&GlobalTransform, &mut Transform, &ChipExtents, Entity), With<Chip>>,
    mut cursor: ResMut<Cursor>,
) {
    if let Some(cursor_world_pos) = cursor.world_pos {
        if let CursorState::DraggingChip(dragged_chip_entity) = cursor.state {
            if input.pressed(MouseButton::Left) {
                for (_, mut chip_transform, _, chip_entity) in q_chips.iter_mut() {
                    if chip_entity != dragged_chip_entity {
                        continue;
                    }

                    chip_transform.translation =
                        cursor_world_pos.extend(chip_transform.translation.z);
                }

                return;
            }

            if input.just_released(MouseButton::Left) {
                cursor.state = CursorState::Idle;
            }
        }

        if input.just_pressed(MouseButton::Left) && cursor.state == CursorState::Idle {
            for (chip_global_transform, _, chip_extents, chip_entity) in q_chips.iter_mut() {
                let chip_position: Vec2 = Vec2::new(
                    chip_global_transform.translation().x,
                    chip_global_transform.translation().y,
                );

                let cursor_on_chip: bool = cursor_world_pos.x
                    >= chip_position.x - (chip_extents.0.x / 2.0)
                    && cursor_world_pos.x <= chip_position.x + (chip_extents.0.x / 2.0)
                    && cursor_world_pos.y >= chip_position.y - (chip_extents.0.y / 2.0)
                    && cursor_world_pos.y <= chip_position.y + (chip_extents.0.y / 2.0);

                if !cursor_on_chip {
                    continue;
                }

                //window.set_cursor_icon(CursorIcon::Grab);
                cursor.state = CursorState::DraggingChip(chip_entity);
                return;
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn drag_board_binary_io(
    input: Res<Input<MouseButton>>,
    mut cursor: ResMut<Cursor>,
    mut q_bbio_handle_bars: Query<
        (
            &GlobalTransform,
            &BoardBinaryIOHandleBarExtents,
            Entity,
            &Parent,
        ),
        With<BoardBinaryIOHandleBar>,
    >,
    mut q_bbio: Query<&mut Transform, Or<(With<BoardBinaryInput>, With<BoardBinaryOutput>)>>,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    if let Some(cursor_world_pos) = cursor.world_pos {
        if let CursorState::DraggingBBIO(dragged_entity) = cursor.state {
            if input.pressed(MouseButton::Left) {
                for (_, _, handle_bar_entity, bbio_entity) in q_bbio_handle_bars.iter() {
                    if handle_bar_entity != dragged_entity {
                        continue;
                    }

                    let mut bbio_transform = q_bbio.get_mut(bbio_entity.get())
                        .expect("BoardBinaryInputHandleBar has no parent BoardBinaryInput or BoardBinaryOutput.");

                    bbio_transform.translation =
                        cursor_world_pos.extend(bbio_transform.translation.z);
                }

                return;
            }

            if input.just_released(MouseButton::Left) {
                cursor.state = CursorState::Idle;
            }
        }

        if input.just_pressed(MouseButton::Left) && cursor.state == CursorState::Idle {
            for (handle_bar_global_transform, handle_bar_extents, handle_bar_entity, _) in
                q_bbio_handle_bars.iter_mut()
            {
                let handle_bar_position: Vec2 = Vec2::new(
                    handle_bar_global_transform.translation().x,
                    handle_bar_global_transform.translation().y,
                );

                let cursor_on_handle_bar: bool = cursor_world_pos.x
                    >= handle_bar_position.x - (handle_bar_extents.0.x / 2.0)
                        + render_settings.binary_io_pin_radius // Workaround weil die handle bar unterm switch liegt
                    && cursor_world_pos.x <= handle_bar_position.x + (handle_bar_extents.0.x / 2.0)
                    && cursor_world_pos.y >= handle_bar_position.y - (handle_bar_extents.0.y / 2.0)
                    && cursor_world_pos.y <= handle_bar_position.y + (handle_bar_extents.0.y / 2.0);

                if !cursor_on_handle_bar {
                    continue;
                }

                //window.set_cursor_icon(CursorIcon::Grab);
                cursor.state = CursorState::DraggingBBIO(handle_bar_entity);
                return;
            }
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn drag_wire(
    input: Res<Input<MouseButton>>,
    q_output_pins: Query<(&GlobalTransform, &Children), (With<ChipOutputPin>, Without<Camera>)>,
    q_input_pins: Query<(&GlobalTransform, Entity), (With<ChipInputPin>, Without<Camera>)>,
    q_board_output_pins: Query<
        (&GlobalTransform, Entity),
        (With<BoardBinaryOutputPin>, Without<Camera>),
    >,
    q_board_input_pins: Query<
        (&GlobalTransform, &Children),
        (With<BoardBinaryInputPin>, Without<Camera>),
    >,
    mut q_wires: Query<(&mut Path, &GlobalTransform, &mut Wire)>,
    mut cursor: ResMut<Cursor>,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    if let Some(cursor_world_pos) = cursor.world_pos {
        if input.just_pressed(MouseButton::Left) && cursor.state == CursorState::Idle {
            for (pin_transform, pin_children) in q_output_pins.iter() {
                if cursor_world_pos.distance(pin_transform.translation().truncate())
                    > render_settings.chip_pin_radius
                {
                    continue;
                }

                // cursor is on pin
                let &wire_entity = pin_children.first().unwrap();
                cursor.state = CursorState::DraggingWire(wire_entity);
                return;
            }

            for (pin_transform, pin_children) in q_board_input_pins.iter() {
                if cursor_world_pos.distance(pin_transform.translation().truncate())
                    > render_settings.binary_io_pin_radius
                {
                    continue;
                }

                // cursor is on pin
                let &wire_entity = pin_children.first().unwrap();
                cursor.state = CursorState::DraggingWire(wire_entity);
                return;
            }
        }

        if let CursorState::DraggingWire(wire_entity) = cursor.state {
            if let Ok(wire_components) = q_wires.get_mut(wire_entity) {
                let (mut path, output_pin_transform, mut wire) = wire_components;
                let mut new_wire = shapes::Line(Vec2::ZERO, Vec2::ZERO);

                //TODO: nicht jedes mal abfrage? aber juckt glaube nicht
                if wire.dest_pin.is_some() {
                    wire.dest_pin = None;
                }

                if input.pressed(MouseButton::Left) {
                    new_wire.1 = cursor_world_pos - output_pin_transform.translation().truncate();
                    *path = ShapePath::build_as(&new_wire);
                } else if input.just_released(MouseButton::Left) {
                    let hovered_chip_pin = || {
                        q_input_pins.iter().find(|pin| {
                            cursor_world_pos.distance(pin.0.translation().truncate())
                                <= render_settings.chip_pin_radius
                        })
                    };

                    let hovered_board_pin = || {
                        q_board_output_pins.iter().find(|pin| {
                            cursor_world_pos.distance(pin.0.translation().truncate())
                                <= render_settings.binary_io_pin_radius
                        })
                    };

                    let hovered_pin = hovered_chip_pin().or_else(hovered_board_pin);

                    if let Some(hovered_pin) = hovered_pin {
                        // connect wire to pin
                        wire.dest_pin = Some(hovered_pin.1);
                        new_wire.1 = hovered_pin.0.translation().truncate()
                            - output_pin_transform.translation().truncate();
                        *path = ShapePath::build_as(&new_wire);
                        cursor.state = CursorState::Idle;
                        return;
                    }

                    // reset wire if dragged on nothing
                    *path = ShapePath::build_as(&new_wire);
                    cursor.state = CursorState::Idle;
                }
            }
        }
    }
}

pub fn toggle_board_input_pin(
    input: Res<Input<MouseButton>>,
    q_inputs: Query<&Children, With<BoardBinaryInput>>,
    q_input_switches: Query<(&GlobalTransform, &Parent), With<BoardBinaryInputSwitch>>,
    mut q_input_pins: Query<(&mut BoardBinaryInputPin, &mut PinState)>,
    render_settings: Res<CircuitBoardRenderingSettings>,
    cursor: Res<Cursor>,
) {
    if let Some(cursor_world_pos) = cursor.world_pos {
        if input.just_pressed(MouseButton::Left) {
            for (switch_transform, parent) in q_input_switches.iter() {
                if cursor_world_pos.distance(switch_transform.translation().truncate())
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

                let (mut board_binary_input_pin, mut board_binary_input_pin_state) = q_input_pins
                    .get_mut(*board_binary_input_pin_entity)
                    .expect("BoardBinaryInput has no BoardBinaryInputPin child.");

                board_binary_input_pin_state.toggle();
                break;
            }
        }
    }
}
//TODO: delete selected / selected marker / drag selected
