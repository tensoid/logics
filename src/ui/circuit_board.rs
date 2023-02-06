use crate::simulation::{
    chip::{Chip, ChipExtents, ChipSpecs, SpawnChipEvent},
    pin::{ChipInputPin, ChipOutputPin, PinRadius},
    pin_state::PinState,
    wire::Wire,
};
use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, render::Shape};

//TODO: maybe make resource
const CHIP_PIN_GAP: f32 = 25.0;
const CHIP_PIN_RADIUS: f32 = 7.0;
const WIRE_LINE_WIDTH: f32 = 1.0;

#[derive(Resource, PartialEq)]
pub enum CursorState {
    Idle,
    DraggingChip(Entity), //TODO: put IsBeingDragged component on entity instead
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

//TODO: define all geometries in a class or smth to clean up
pub fn spawn_chip_event(
    mut spawn_ev: EventReader<SpawnChipEvent>,
    mut commands: Commands,
    chip_specs: Res<ChipSpecs>,
) {
    for ev in spawn_ev.iter() {
        //TODO: take max of input and output pins and adjust size accordingly

        let chip_spec = chip_specs
            .0
            .iter()
            .find(|spec| spec.name == ev.chip_name)
            .unwrap();

        let num_input_pins = ChipInputPin::num_input_pins_from_chip_spec(chip_spec);

        let chip_extents: Vec2 = Vec2::new(
            CHIP_PIN_GAP * (num_input_pins + 1) as f32,
            CHIP_PIN_GAP * (num_input_pins + 1) as f32,
        );

        let chip_shape = shapes::Rectangle {
            extents: chip_extents,
            ..default()
        };

        let pin_shape = shapes::Circle {
            radius: CHIP_PIN_RADIUS,
            ..default()
        };

        let wire_shape = shapes::Line(Vec2::ZERO, Vec2::ZERO);

        commands
            .spawn(GeometryBuilder::build_as(
                &chip_shape,
                DrawMode::Fill(FillMode {
                    options: FillOptions::default(),
                    color: Color::YELLOW,
                }),
                Transform::from_xyz(ev.position.x, ev.position.y, 0.0),
            ))
            .insert(Chip)
            .insert(ChipExtents(chip_extents))
            .insert(chip_spec.clone())
            .with_children(|chip| {
                for i in 0..num_input_pins {
                    chip.spawn(GeometryBuilder::build_as(
                        &pin_shape,
                        DrawMode::Fill(FillMode {
                            options: FillOptions::default(),
                            color: Color::RED,
                        }),
                        Transform::from_xyz(
                            -(chip_extents.x / 2.0) - CHIP_PIN_RADIUS,
                            (i as f32 * CHIP_PIN_GAP) - (chip_extents.y / 2.0) + CHIP_PIN_GAP,
                            0.0,
                        ),
                    ))
                    .insert(ChipInputPin(PinState::Low));
                }

                chip.spawn(GeometryBuilder::build_as(
                    &pin_shape,
                    DrawMode::Fill(FillMode {
                        options: FillOptions::default(),
                        color: Color::RED,
                    }),
                    Transform::from_xyz((chip_extents.x / 2.0) + CHIP_PIN_RADIUS, 0.0, 0.0),
                ))
                .insert(ChipOutputPin(PinState::Low))
                .insert(PinRadius(CHIP_PIN_RADIUS))
                .with_children(|pin| {
                    pin.spawn(GeometryBuilder::build_as(
                        &wire_shape,
                        DrawMode::Stroke(StrokeMode::new(Color::GREEN, WIRE_LINE_WIDTH)),
                        Transform::IDENTITY,
                    ))
                    .insert(Wire {
                        dest_pin: Option::None,
                    });
                });
            });
    }
}

pub fn screen_to_world_space(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    position: Vec2,
) -> Vec2 {
    let window_size = Vec2::new(window.width(), window.height());
    let ndc = (position / window_size) * 2.0 - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    ndc_to_world.project_point3(ndc.extend(-1.0)).truncate()
}

pub fn drag_chip(
    input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), Without<Chip>>,
    mut q_chips: Query<
        (&GlobalTransform, &mut Transform, &ChipExtents, Entity),
        (With<Chip>, Without<Camera>),
    >,
    mut cursor_state: ResMut<CursorState>,
) {
    let window = windows.get_primary().unwrap();
    let (camera, camera_transform) = q_camera.single();

    if let Some(cursor_screen_pos) = window.cursor_position() {
        let cursor_position: Vec2 =
            screen_to_world_space(window, camera, camera_transform, cursor_screen_pos);

        if input.just_pressed(MouseButton::Left) {
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

                *cursor_state = CursorState::DraggingChip(chip_entity);
                return;
            }
        }

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
            }
        }
    }
}

pub fn drag_wire(
    input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), Without<Chip>>,
    mut q_pins: Query<
        (&GlobalTransform, &PinRadius, &Children),
        (With<ChipOutputPin>, Without<Camera>),
    >,
    mut q_wires: Query<(&mut Path, &GlobalTransform), With<Wire>>,
    mut cursor_state: ResMut<CursorState>,
) {
    let window = windows.get_primary().unwrap();
    let (camera, camera_transform) = q_camera.single();

    if let Some(cursor_screen_pos) = window.cursor_position() {
        let cursor_position: Vec2 =
            screen_to_world_space(window, camera, camera_transform, cursor_screen_pos);

        if input.just_pressed(MouseButton::Left) {
            for (pin_transform, pin_radius, pin_children) in q_pins.iter() {
                if cursor_position.distance(pin_transform.translation().truncate()) > pin_radius.0 {
                    continue;
                }

                // cursor is on pin
                let &wire_entity = pin_children.first().unwrap();
                *cursor_state = CursorState::DraggingWire(wire_entity);
                break;
            }
        }

        if let CursorState::DraggingWire(wire_entity) = *cursor_state {
            let (mut path, pin_transform) = q_wires.get_mut(wire_entity).unwrap();
            let mut new_wire = shapes::Line(Vec2::ZERO, Vec2::ZERO);

            if input.pressed(MouseButton::Left) {
                new_wire.1 = cursor_position - pin_transform.translation().truncate();
                *path = ShapePath::build_as(&new_wire);
            } else if input.just_released(MouseButton::Left) {
                // TODO: check if over input pin, and set in wire dest pin

                // if not reset
                *path = ShapePath::build_as(&new_wire);
                *cursor_state = CursorState::Idle;
            }
        }
    }
}
