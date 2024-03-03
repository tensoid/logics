use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::get_cursor_mut;
use crate::{events::events::SpawnChipEvent, simulation::expressions::Expr};

use crate::designer::{
    board_entity::BoardEntity, bounding_box::BoundingBox, draw_layer::DrawLayer,
    render_settings::CircuitBoardRenderingSettings, signal_state::SignalState,
};

use super::cursor::{Cursor, CursorState};

#[derive(Component)]
pub struct Chip;

#[derive(Component)]
pub struct ChipExtents(pub Vec2);

#[derive(Resource)]
pub struct ChipSpecs(pub Vec<ChipSpec>);

#[derive(Component, Clone)]
pub struct ChipSpec {
    pub name: String,
    pub expression: Expr,
}

#[derive(Component)]
pub struct ChipInputPin;

#[derive(Component)]
pub struct ChipOutputPin;

pub fn spawn_chip_event(
    mut spawn_ev: EventReader<SpawnChipEvent>,
    mut commands: Commands,
    chip_specs: Res<ChipSpecs>,
    asset_server: Res<AssetServer>,
    render_settings: Res<CircuitBoardRenderingSettings>,
    mut q_cursor: Query<(Entity, &mut Cursor)>,
) {
    let (cursor_entity, mut cursor) = get_cursor_mut!(q_cursor);

    for ev in spawn_ev.read() {
        let chip_spec = chip_specs
            .0
            .iter()
            .find(|spec| spec.name == ev.chip_name)
            .unwrap();

        let num_input_pins = chip_spec.expression.1;

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

        let font: Handle<Font> = asset_server.load("fonts/VCR_OSD_MONO.ttf");

        let text_style = TextStyle {
            font_size: 20.0,
            color: Color::BLACK,
            font,
        };

        let chip_entity = commands
            .spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&chip_shape),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(
                            ev.position.x,
                            ev.position.y,
                            DrawLayer::Chip.get_z(),
                        ),
                        ..default()
                    },
                    ..default()
                },
                Fill::color(render_settings.chip_color),
                Stroke::new(Color::BLACK, 1.0),
                Chip,
                ChipExtents(chip_extents),
                BoundingBox::new(chip_extents / 2.0, true),
                chip_spec.clone(),
                BoardEntity,
            ))
            .with_children(|chip| {
                //Chip Name
                chip.spawn(Text2dBundle {
                    text: Text::from_section(ev.chip_name.to_uppercase(), text_style)
                        .with_justify(JustifyText::Center),
                    transform: Transform::from_xyz(0.0, 0.0, DrawLayer::ChipName.get_z()),
                    ..default()
                });

                // Input pins
                for i in 0..num_input_pins {
                    chip.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&pin_shape),
                            spatial: SpatialBundle {
                                transform: Transform::from_xyz(
                                    -(chip_extents.x / 2.0),
                                    (i as f32 * render_settings.chip_pin_gap)
                                        - (chip_extents.y / 2.0)
                                        + render_settings.chip_pin_gap,
                                    DrawLayer::Pin.get_z(),
                                ),
                                ..default()
                            },
                            ..default()
                        },
                        Fill::color(render_settings.signal_low_color),
                        ChipInputPin,
                        SignalState::Low,
                    ));
                }

                // Output pins
                chip.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&pin_shape),
                        spatial: SpatialBundle {
                            transform: Transform::from_xyz(
                                chip_extents.x / 2.0,
                                0.0,
                                DrawLayer::Pin.get_z(),
                            ),
                            ..default()
                        },
                        ..default()
                    },
                    Fill::color(render_settings.signal_low_color),
                    ChipOutputPin,
                    SignalState::Low,
                ));
            })
            .id();

        // Parent chip to curser and start drag
        cursor.state = CursorState::Dragging;
        commands.entity(cursor_entity).add_child(chip_entity);
    }
}
