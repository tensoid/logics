use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::events::events::SpawnBoardEntityEvent;
use crate::get_cursor_mut;
use crate::simulation::expressions::Expr;

use crate::designer::{
    board_entity::BoardEntity, bounding_box::BoundingBox,
    render_settings::CircuitBoardRenderingSettings, signal_state::SignalState,
};

use super::cursor::Cursor;

#[derive(Component)]
pub struct Chip;

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

pub fn spawn_chip(
    mut spawn_ev: EventReader<SpawnBoardEntityEvent>,
    mut commands: Commands,
    chip_specs: Res<ChipSpecs>,
    asset_server: Res<AssetServer>,
    render_settings: Res<CircuitBoardRenderingSettings>,
    mut q_cursor: Query<(Entity, &mut Cursor)>,
) -> Option<(Entity, SpawnBoardEntityEvent)> {
    let (cursor_entity, mut cursor) = get_cursor_mut!(q_cursor);

    for ev in spawn_ev.read() {
        let Some(chip_spec) = chip_specs.0.iter().find(|spec| spec.name == ev.name) else {
            continue;
        };

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
                        transform: Transform::IDENTITY,
                        ..default()
                    },
                    ..default()
                },
                Fill::color(render_settings.chip_color),
                Stroke::new(
                    render_settings.board_entity_stroke_color,
                    render_settings.board_entity_stroke_width,
                ),
                Chip,
                BoundingBox::rect_new(chip_extents / 2.0, true),
                chip_spec.clone(),
                BoardEntity,
            ))
            .with_children(|chip| {
                //Chip Name
                chip.spawn(Text2dBundle {
                    text: Text::from_section(ev.name.to_uppercase(), text_style)
                        .with_justify(JustifyText::Center),
                    transform: Transform::from_xyz(0.0, 0.0, 0.01),
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
                                    0.01,
                                ),
                                ..default()
                            },
                            ..default()
                        },
                        Fill::color(render_settings.pin_color),
                        ChipInputPin,
                        SignalState::Low,
                        BoundingBox::circle_new(render_settings.binary_io_pin_radius, false),
                    ));
                }

                // Output pins
                chip.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&pin_shape),
                        spatial: SpatialBundle {
                            transform: Transform::from_xyz(chip_extents.x / 2.0, 0.0, 0.01),
                            ..default()
                        },
                        ..default()
                    },
                    Fill::color(render_settings.pin_color),
                    ChipOutputPin,
                    SignalState::Low,
                    BoundingBox::circle_new(render_settings.binary_io_pin_radius, false),
                ));
            })
            .id();

        return Some((chip_entity, ev.clone()));
    }

    None
}
