use crate::simulation::{
    chip::{Chip, ChipExtents, ChipSpecs, SpawnChipEvent},
    pin::{ChipInputPin, ChipOutputPin},
    pin_state::PinState,
    wire::Wire,
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::draw_layer::DrawLayer;

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
    asset_server: Res<AssetServer>,
) {
    for ev in spawn_ev.iter() {
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

        let font: Handle<Font> = asset_server.load("fonts/OpenSans-ExtraBold.ttf");

        let text_style = TextStyle {
            font_size: 30.0,
            color: Color::BLACK,
            font,
        };

        commands
            .spawn(GeometryBuilder::build_as(
                &chip_shape,
                DrawMode::Fill(FillMode {
                    options: FillOptions::default(),
                    color: Color::YELLOW,
                }),
                Transform::from_xyz(ev.position.x, ev.position.y, DrawLayer::Chip.get_z()),
            ))
            .insert(Chip)
            .insert(ChipExtents(chip_extents))
            .insert(chip_spec.clone())
            .with_children(|chip| {
                //Chip Name
                chip.spawn(Text2dBundle {
                    text: Text::from_section(&ev.chip_name.to_uppercase(), text_style)
                        .with_alignment(TextAlignment::CENTER),
                    transform: Transform::from_xyz(0.0, 0.0, DrawLayer::ChipName.get_z()),
                    ..default()
                });

                // Input pins
                for i in 0..num_input_pins {
                    chip.spawn(GeometryBuilder::build_as(
                        &pin_shape,
                        DrawMode::Fill(FillMode {
                            options: FillOptions::default(),
                            color: Color::RED,
                        }),
                        Transform::from_xyz(
                            -(chip_extents.x / 2.0),
                            (i as f32 * CHIP_PIN_GAP) - (chip_extents.y / 2.0) + CHIP_PIN_GAP,
                            DrawLayer::Pin.get_z(),
                        ),
                    ))
                    .insert(ChipInputPin(PinState::Low));
                }

                // Output pins
                chip.spawn(GeometryBuilder::build_as(
                    &pin_shape,
                    DrawMode::Fill(FillMode {
                        options: FillOptions::default(),
                        color: Color::RED,
                    }),
                    Transform::from_xyz(chip_extents.x / 2.0, 0.0, DrawLayer::Pin.get_z()),
                ))
                .insert(ChipOutputPin(PinState::Low))
                .with_children(|pin| {
                    pin.spawn(GeometryBuilder::build_as(
                        &wire_shape,
                        DrawMode::Stroke(StrokeMode::new(Color::RED, WIRE_LINE_WIDTH)),
                        Transform::from_xyz(0.0, 0.0, DrawLayer::Wire.get_z()),
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

                //window.set_cursor_icon(CursorIcon::Grab);
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
                //window.set_cursor_icon(CursorIcon::Default);
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
) {
    let window = windows.get_primary().unwrap();
    let (camera, camera_transform) = q_camera.single();

    if let Some(cursor_screen_pos) = window.cursor_position() {
        let cursor_position: Vec2 =
            screen_to_world_space(window, camera, camera_transform, cursor_screen_pos);

        if input.just_pressed(MouseButton::Left) {
            for (pin_transform, pin_children) in q_output_pins.iter() {
                if cursor_position.distance(pin_transform.translation().truncate())
                    > CHIP_PIN_RADIUS
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
            let (mut path, output_pin_transform, mut wire) = q_wires.get_mut(wire_entity).unwrap();
            let mut new_wire = shapes::Line(Vec2::ZERO, Vec2::ZERO);

            if input.pressed(MouseButton::Left) {
                new_wire.1 = cursor_position - output_pin_transform.translation().truncate();
                *path = ShapePath::build_as(&new_wire);
            } else if input.just_released(MouseButton::Left) {
                for (input_pin_transform, pin_entity) in q_input_pins.iter() {
                    if cursor_position.distance(input_pin_transform.translation().truncate())
                        > CHIP_PIN_RADIUS
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

pub fn update_wires(
    q_moved_chips: Query<(Entity, &Children), (With<Chip>, Changed<GlobalTransform>)>,
    q_output_pins: Query<(&GlobalTransform, &Children), (With<ChipOutputPin>, Without<Camera>)>,
    q_input_pins: Query<&GlobalTransform, (With<ChipInputPin>, Without<Camera>)>,
    mut q_wires: Query<(&Wire, &mut Path, &GlobalTransform)>,
) {
    for (_, chip_children) in q_moved_chips.iter() {
        // Output pins
        for &output_pin_entity in chip_children.iter() {
            if let Ok(output_pin) = q_output_pins.get(output_pin_entity) {
                let output_pin_wire_entity = output_pin.1.first().unwrap();
                let (wire, mut wire_path, _) = q_wires.get_mut(*output_pin_wire_entity).unwrap();
                if let Some(wire_dest_pin_entity) = wire.dest_pin {
                    let wire_dest_pin_transform = q_input_pins.get(wire_dest_pin_entity).unwrap();
                    let new_wire = shapes::Line(
                        Vec2::ZERO,
                        wire_dest_pin_transform.translation().truncate()
                            - output_pin.0.translation().truncate(),
                    );
                    *wire_path = ShapePath::build_as(&new_wire);
                }
            }
        }

        // Input pins
        for &input_pin_entity in chip_children.iter() {
            if let Ok(input_pin_transform) = q_input_pins.get(input_pin_entity) {
                for (wire, mut wire_path, wire_transform) in q_wires.iter_mut() {
                    if let Some(wire_dest_pin) = wire.dest_pin {
                        if wire_dest_pin != input_pin_entity {
                            continue;
                        }

                        let new_wire = shapes::Line(
                            Vec2::ZERO,
                            input_pin_transform.translation().truncate()
                                - wire_transform.translation().truncate(),
                        );
                        *wire_path = ShapePath::build_as(&new_wire);
                    }
                }
            }
        }
    }
}

pub fn delete_chip(
    input: Res<Input<KeyCode>>,
    q_chips: Query<(Entity, &GlobalTransform, &ChipExtents), With<Chip>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let (camera, camera_transform) = q_camera.single();

    if let Some(cursor_screen_pos) = window.cursor_position() {
        let cursor_position: Vec2 =
            screen_to_world_space(window, camera, camera_transform, cursor_screen_pos);

        if input.just_pressed(KeyCode::D) {
            for (chip_entity, chip_transform, chip_extents) in q_chips.iter() {
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

                commands.entity(chip_entity).despawn_recursive();
            }
        }
    }
}
