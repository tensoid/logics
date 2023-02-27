use crate::simulation::{
    chip::{Chip, ChipExtents, ChipSpecs, SpawnChipEvent},
    pin::{BoardInputPin, BoardOutputPin, ChipInputPin, ChipOutputPin, SpawnIOPinEvent},
    pin_state::PinState,
    wire::Wire,
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::draw_layer::DrawLayer;

#[derive(Resource)]
pub struct CircuitBoardRenderingSettings {
    pub chip_pin_gap: f32,
    pub chip_pin_radius: f32,
    pub io_pin_radius: f32,
    pub wire_line_width: f32,
    //TODO: shapes maybe
}

//TODO: define all geometries in a class or smth to clean up
pub fn spawn_chip_event(
    mut spawn_ev: EventReader<SpawnChipEvent>,
    mut commands: Commands,
    chip_specs: Res<ChipSpecs>,
    asset_server: Res<AssetServer>,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    for ev in spawn_ev.iter() {
        let chip_spec = chip_specs
            .0
            .iter()
            .find(|spec| spec.name == ev.chip_name)
            .unwrap();

        let num_input_pins = ChipInputPin::num_input_pins_from_chip_spec(chip_spec);

        let chip_extents: Vec2 = Vec2::new(
            render_settings.chip_pin_gap * (num_input_pins + 1) as f32,
            render_settings.chip_pin_gap * (num_input_pins + 1) as f32,
        );

        let chip_shape = shapes::Rectangle {
            extents: chip_extents,
            ..default()
        };

        let pin_shape = shapes::Circle {
            radius: render_settings.chip_pin_radius,
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
                            (i as f32 * render_settings.chip_pin_gap) - (chip_extents.y / 2.0)
                                + render_settings.chip_pin_gap,
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
                        DrawMode::Stroke(StrokeMode::new(
                            Color::RED,
                            render_settings.wire_line_width,
                        )),
                        Transform::from_xyz(0.0, 0.0, DrawLayer::Wire.get_z()),
                    ))
                    .insert(Wire {
                        dest_pin: Option::None,
                    });
                });
            });
    }
}

pub fn spawn_io_pin_event(
    mut commands: Commands,
    mut spawn_ev: EventReader<SpawnIOPinEvent>,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    for ev in spawn_ev.iter() {
        let pin_shape = shapes::Circle {
            radius: render_settings.io_pin_radius,
            ..default()
        };

        let pin_entity = commands
            .spawn(GeometryBuilder::build_as(
                &pin_shape,
                DrawMode::Fill(FillMode {
                    options: FillOptions::default(),
                    color: Color::RED,
                }),
                Transform::from_xyz(ev.position.x, ev.position.y, DrawLayer::Pin.get_z()),
            ))
            .id();

        if ev.is_input {
            commands
                .entity(pin_entity)
                .insert(BoardInputPin(PinState::Low));
        } else {
            commands
                .entity(pin_entity)
                .insert(BoardOutputPin(PinState::Low));
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
