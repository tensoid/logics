use crate::simulation::{
    chip::{Chip, ChipExtents, ChipSpecs, SpawnChipEvent},
    pin::{BoardInputPin, ChipInputPin, ChipOutputPin, SpawnIOPinEvent},
    pin_state::PinState,
    wire::Wire,
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::{
    circuit_board::CircuitBoardRenderingSettings, draw_layer::DrawLayer,
    utils::screen_to_world_space,
};

#[derive(Resource, PartialEq)]
pub enum CursorState {
    Idle,
    DraggingChip(Entity), //TODO: put IsBeingDragged component on entity instead or is selected marker component
    DraggingWire(Entity),
}

pub fn spawn_chip_at_cursor(
    input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut ev_writer: EventWriter<SpawnChipEvent>,
) {
    if input.just_pressed(MouseButton::Right) {
        let window = windows.get_primary().unwrap();
        if let Some(cursor_screen_pos) = window.cursor_position() {
            let (camera, camera_transform) = q_camera.single();

            let cursor_world_pos: Vec2 =
                screen_to_world_space(window, camera, camera_transform, cursor_screen_pos);

            ev_writer.send(SpawnChipEvent {
                chip_name: "and".to_string(),
                position: cursor_world_pos,
            });
        }
    }
}

pub fn spawn_io_pin_at_cursor(
    input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut ev_writer: EventWriter<SpawnIOPinEvent>,
) {
    let spawn_input_pin = if input.just_pressed(KeyCode::I) {
        true
    } else if input.just_pressed(KeyCode::O) {
        false
    } else {
        return;
    };

    let window = windows.get_primary().unwrap();
    if let Some(cursor_screen_pos) = window.cursor_position() {
        let (camera, camera_transform) = q_camera.single();

        let cursor_world_pos: Vec2 =
            screen_to_world_space(window, camera, camera_transform, cursor_screen_pos);

        ev_writer.send(SpawnIOPinEvent {
            is_input: spawn_input_pin,
            position: cursor_world_pos,
        });
    }
}

pub fn delete_chip(
    input: Res<Input<KeyCode>>,
    q_chips: Query<(Entity, &GlobalTransform, &ChipExtents, &Children), With<Chip>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    windows: Res<Windows>,
    mut q_wires: Query<(&mut Wire, &mut Path), With<Wire>>,
    cursor_state: Res<CursorState>,
) {
    let window = windows.get_primary().unwrap();
    let (camera, camera_transform) = q_camera.single();

    if *cursor_state != CursorState::Idle {
        return;
    }

    if let Some(cursor_screen_pos) = window.cursor_position() {
        let cursor_position: Vec2 =
            screen_to_world_space(window, camera, camera_transform, cursor_screen_pos);

        if input.just_pressed(KeyCode::D) {
            for (chip_entity, chip_transform, chip_extents, chip_children) in q_chips.iter() {
                let chip_position: Vec2 = Vec2::new(
                    chip_transform.translation().x,
                    chip_transform.translation().y,
                );

                let cursor_on_chip: bool = cursor_position.x
                    >= chip_position.x - (chip_extents.0.x / 2.0)
                    && cursor_position.x <= chip_position.x + (chip_extents.0.x / 2.0)
                    && cursor_position.y >= chip_position.y - (chip_extents.0.y / 2.0)
                    && cursor_position.y <= chip_position.y + (chip_extents.0.y / 2.0);

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
    windows: ResMut<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), Without<Chip>>,
    mut q_chips: Query<
        (&GlobalTransform, &mut Transform, &ChipExtents, Entity),
        (With<Chip>, Without<Camera>),
    >,
    mut cursor_state: ResMut<CursorState>,
    mut commands: Commands,
) {
    let window = windows.get_primary().unwrap();
    let (camera, camera_transform) = q_camera.single();

    if let Some(cursor_screen_pos) = window.cursor_position() {
        let cursor_position: Vec2 =
            screen_to_world_space(window, camera, camera_transform, cursor_screen_pos);

        if let CursorState::DraggingChip(dragged_chip_entity) = *cursor_state {
            if input.pressed(MouseButton::Left) {
                for (_, mut chip_transform, _, chip_entity) in q_chips.iter_mut() {
                    if chip_entity != dragged_chip_entity {
                        continue;
                    }

                    chip_transform.translation =
                        cursor_position.extend(chip_transform.translation.z);
                }

                return;
            }

            if input.just_released(MouseButton::Left) {
                *cursor_state = CursorState::Idle;
                //window.set_cursor_icon(CursorIcon::Default);
            }
        }

        if input.just_pressed(MouseButton::Left) && *cursor_state == CursorState::Idle {
            for (chip_global_transform, _, chip_extents, chip_entity) in q_chips.iter_mut() {
                let chip_position: Vec2 = Vec2::new(
                    chip_global_transform.translation().x,
                    chip_global_transform.translation().y,
                );

                let cursor_on_chip: bool = cursor_position.x
                    >= chip_position.x - (chip_extents.0.x / 2.0)
                    && cursor_position.x <= chip_position.x + (chip_extents.0.x / 2.0)
                    && cursor_position.y >= chip_position.y - (chip_extents.0.y / 2.0)
                    && cursor_position.y <= chip_position.y + (chip_extents.0.y / 2.0);

                if !cursor_on_chip {
                    continue;
                }

                //window.set_cursor_icon(CursorIcon::Grab);
                *cursor_state = CursorState::DraggingChip(chip_entity);
                return;
            }
        }
    }
}

pub fn drag_wire(
    input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), Without<Chip>>,
    q_output_pins: Query<(&GlobalTransform, &Children), (With<ChipOutputPin>, Without<Camera>)>,
    q_input_pins: Query<(&GlobalTransform, Entity), (With<ChipInputPin>, Without<Camera>)>,
    mut q_wires: Query<(&mut Path, &GlobalTransform, &mut Wire)>,
    mut cursor_state: ResMut<CursorState>,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    let window = windows.get_primary().unwrap();
    let (camera, camera_transform) = q_camera.single();

    if let Some(cursor_screen_pos) = window.cursor_position() {
        let cursor_position: Vec2 =
            screen_to_world_space(window, camera, camera_transform, cursor_screen_pos);

        if input.just_pressed(MouseButton::Left) && *cursor_state == CursorState::Idle {
            for (pin_transform, pin_children) in q_output_pins.iter() {
                if cursor_position.distance(pin_transform.translation().truncate())
                    > render_settings.chip_pin_radius
                {
                    continue;
                }

                // cursor is on pin
                let &wire_entity = pin_children.first().unwrap();
                *cursor_state = CursorState::DraggingWire(wire_entity);
                break;
            }
        }

        if let CursorState::DraggingWire(wire_entity) = *cursor_state {
            if let Ok(wire_components) = q_wires.get_mut(wire_entity) {
                let (mut path, output_pin_transform, mut wire) = wire_components;
                let mut new_wire = shapes::Line(Vec2::ZERO, Vec2::ZERO);

                if input.pressed(MouseButton::Left) {
                    new_wire.1 = cursor_position - output_pin_transform.translation().truncate();
                    *path = ShapePath::build_as(&new_wire);
                } else if input.just_released(MouseButton::Left) {
                    for (input_pin_transform, pin_entity) in q_input_pins.iter() {
                        if cursor_position.distance(input_pin_transform.translation().truncate())
                            > render_settings.chip_pin_radius
                        {
                            continue;
                        }

                        // connect wire to pin
                        wire.dest_pin = Some(pin_entity);
                        new_wire.1 = input_pin_transform.translation().truncate()
                            - output_pin_transform.translation().truncate();
                        *path = ShapePath::build_as(&new_wire);
                        *cursor_state = CursorState::Idle;
                        return;
                    }

                    // reset wire if dragged on nothing
                    wire.dest_pin = None;
                    *path = ShapePath::build_as(&new_wire);
                    *cursor_state = CursorState::Idle;
                }
            }
        }
    }
}

//TODO: delete selected / selected marker / drag selected
