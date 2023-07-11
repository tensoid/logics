use crate::simulation::{
    chip::{Chip, ChipExtents, SpawnChipEvent},
    pin::{BoardInputPin, BoardOutputPin, ChipInputPin, ChipOutputPin, SpawnIOPinEvent},
    pin_state::PinState,
    wire::Wire,
};
use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::egui::Key;
use bevy_prototype_lyon::prelude::*;

use super::{circuit_board::CircuitBoardRenderingSettings, utils::screen_to_world_space};

#[derive(PartialEq)]
pub enum CursorState {
    Idle,
    DraggingChip(Entity), //TODO: put IsBeingDragged component on entity instead or is selected marker component
    DraggingWire(Entity),
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
    input: Res<Input<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), Without<Chip>>,
    mut cursor: ResMut<Cursor>,
    mut mouse_motion_ev: EventReader<MouseMotion>,
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

pub fn drag_chip(
    input: Res<Input<MouseButton>>,
    q_camera: Query<(&Camera, &GlobalTransform), Without<Chip>>,
    mut q_chips: Query<
        (&GlobalTransform, &mut Transform, &ChipExtents, Entity),
        (With<Chip>, Without<Camera>),
    >,
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

pub fn drag_wire(
    input: Res<Input<MouseButton>>,
    q_output_pins: Query<(&GlobalTransform, &Children), (With<ChipOutputPin>, Without<Camera>)>,
    q_input_pins: Query<(&GlobalTransform, Entity), (With<ChipInputPin>, Without<Camera>)>,
    q_board_output_pins: Query<(&GlobalTransform, Entity), (With<BoardOutputPin>, Without<Camera>)>,
    q_board_input_pins: Query<
        (&GlobalTransform, &Children),
        (With<BoardInputPin>, Without<Camera>),
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
                    > render_settings.io_pin_radius
                {
                    continue;
                }

                // cursor is on pin
                let &wire_entity = pin_children.first().unwrap();
                cursor.state = CursorState::DraggingWire(wire_entity);
                return;
            }
        }

        //TODO: drag to board output pin

        if let CursorState::DraggingWire(wire_entity) = cursor.state {
            if let Ok(wire_components) = q_wires.get_mut(wire_entity) {
                let (mut path, output_pin_transform, mut wire) = wire_components;
                let mut new_wire = shapes::Line(Vec2::ZERO, Vec2::ZERO);

                //TODO: nicht jedes mal abfrage? aber juckt glaube nicht
                if wire.dest_pin != None {
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
                                <= render_settings.io_pin_radius
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
    input: Res<Input<KeyCode>>,
    mut q_input_pins: Query<(&GlobalTransform, &mut BoardInputPin, &mut Fill)>,
    render_settings: Res<CircuitBoardRenderingSettings>,
    cursor: Res<Cursor>,
) {
    if let Some(cursor_world_pos) = cursor.world_pos {
        if input.just_pressed(KeyCode::Space) {
            for (pin_transform, mut input_pin, mut fill) in q_input_pins.iter_mut() {
                if cursor_world_pos.distance(pin_transform.translation().truncate())
                    > render_settings.io_pin_radius
                {
                    continue;
                }

                let new_pin_state = match input_pin.0 {
                    PinState::High => PinState::Low,
                    PinState::Low => PinState::High,
                };

                let new_fill = Fill::color(match new_pin_state {
                    PinState::High => Color::GREEN,
                    PinState::Low => Color::RED,
                });

                *fill = new_fill;
                input_pin.0 = new_pin_state;
                break;
            }
        }
    }
}
//TODO: delete selected / selected marker / drag selected
