use bevy::{prelude::*, render::render_phase::Draw};
use bevy_prototype_lyon::prelude::*;

use crate::{events::events::SpawnIOPinEvent, get_cursor_mut};

use super::{
    board_entity::BoardEntity,
    bounding_box::BoundingBox,
    cursor::{Cursor, CursorState},
    render_settings::CircuitBoardRenderingSettings,
    signal_state::SignalState,
};

#[derive(Component)]
pub struct BoardBinaryInput;

#[derive(Component)]
pub struct BoardBinaryInputPin;

#[derive(Component)]
pub struct BoardBinaryInputSwitch;

#[derive(Component)]
pub struct BoardBinaryOutput;

#[derive(Component)]
pub struct BoardBinaryOutputPin;

#[derive(Component)]
pub struct BoardBinaryDisplay;

pub fn spawn_board_binary_input(
    mut commands: Commands,
    mut spawn_ev: EventReader<SpawnIOPinEvent>,
    render_settings: Res<CircuitBoardRenderingSettings>,
    mut q_cursor: Query<(Entity, &mut Cursor)>,
    asset_server: Res<AssetServer>,
) {
    let (cursor_entity, mut cursor) = get_cursor_mut!(q_cursor);

    for ev in spawn_ev.read() {
        if !ev.is_input {
            return;
        }

        let binary_input_extents = Vec2::new(60.0, 30.0);

        let font: Handle<Font> = asset_server.load("fonts/VCR_OSD_MONO.ttf");

        let text_style = TextStyle {
            font_size: 20.0,
            color: Color::BLACK,
            font,
        };

        let pin_bundle = (
            BoardBinaryInputPin,
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: render_settings.binary_io_pin_radius,
                    ..default()
                }),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(binary_input_extents.x / 2.0, 0.0, 0.02),
                    ..default()
                },
                ..default()
            },
            Fill::color(render_settings.signal_low_color),
            SignalState::Low,
        );

        let binary_display_bundle = (
            BoardBinaryDisplay,
            Text2dBundle {
                text: Text::from_section("0", text_style).with_justify(JustifyText::Center),
                transform: Transform::from_xyz(binary_input_extents.x / 4.0, 0.0, 0.01),
                ..default()
            },
        );

        let binary_switch_bundle = (
            BoardBinaryInputSwitch,
            SpriteBundle {
                texture: asset_server.load("images/switch.png"),
                transform: Transform::from_xyz(-binary_input_extents.x / 4.0, 0.0, 0.01),
                sprite: Sprite {
                    custom_size: Some(binary_input_extents / Vec2::new(2.0, 1.0) * 0.8),
                    ..default()
                },
                ..default()
            },
            BoundingBox::rect_new(binary_input_extents / Vec2::new(4.0, 2.0), false),
        );

        let board_binary_input_bundle = (
            BoardBinaryInput,
            BoardEntity,
            BoundingBox::rect_with_offset(
                binary_input_extents / Vec2::new(4.0, 2.0),
                Vec2::new(binary_input_extents.x / 4.0, 0.0),
                true,
            ),
            Fill::color(render_settings.binary_io_color),
            Stroke::new(
                render_settings.binary_io_stroke_color,
                render_settings.binary_io_stroke_width,
            ),
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: binary_input_extents,
                    ..default()
                }),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(ev.position.x, ev.position.y, 0.0),
                    ..default()
                },
                ..default()
            },
        );

        let entity = commands
            .spawn(board_binary_input_bundle)
            .with_children(|bbi| {
                bbi.spawn(binary_switch_bundle);
                bbi.spawn(binary_display_bundle);
                bbi.spawn(pin_bundle);
            })
            .id();

        //Parent io pin to curser and start drag
        cursor.state = CursorState::Dragging;
        commands.entity(cursor_entity).add_child(entity);
    }
}

pub fn spawn_board_binary_output(
    mut commands: Commands,
    mut spawn_ev: EventReader<SpawnIOPinEvent>,
    render_settings: Res<CircuitBoardRenderingSettings>,
    mut q_cursor: Query<(Entity, &mut Cursor)>,
    asset_server: Res<AssetServer>,
) {
    let (cursor_entity, mut cursor) = get_cursor_mut!(q_cursor);

    for ev in spawn_ev.read() {
        if ev.is_input {
            return;
        }

        let binary_output_extents = Vec2::new(30.0, 30.0);

        let font: Handle<Font> = asset_server.load("fonts/VCR_OSD_MONO.ttf");

        let text_style = TextStyle {
            font_size: 20.0,
            color: Color::BLACK,
            font,
        };

        let rect_shape = shapes::Rectangle {
            extents: Vec2::new(30.0, 30.0),
            ..default()
        };

        let pin_bundle = (
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: render_settings.binary_io_pin_radius,
                    ..default()
                }),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(-rect_shape.extents.x / 2.0, 0.0, 0.01),
                    ..default()
                },
                ..default()
            },
            Fill::color(render_settings.signal_low_color),
            BoardBinaryOutputPin,
            SignalState::Low,
        );

        let binary_display_bundle = (
            BoardBinaryDisplay,
            Text2dBundle {
                text: Text::from_section("0", text_style).with_justify(JustifyText::Center),
                transform: Transform::from_xyz(0.0, 0.0, 0.01),
                ..default()
            },
        );

        let board_binary_output_bundle = (
            BoardBinaryOutput,
            BoardEntity,
            BoundingBox::rect_new(rect_shape.extents / 2.0, true),
            Stroke::new(
                render_settings.binary_io_stroke_color,
                render_settings.binary_io_stroke_width,
            ),
            Fill::color(render_settings.binary_io_color),
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: binary_output_extents,
                    ..default()
                }),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(ev.position.x, ev.position.y, 0.0),
                    ..default()
                },
                ..default()
            },
        );

        let entity = commands
            .spawn(board_binary_output_bundle)
            .with_children(|bbo| {
                bbo.spawn(binary_display_bundle);
                bbo.spawn(pin_bundle);
            })
            .id();

        //Parent io pin to curser and start drag
        cursor.state = CursorState::Dragging;
        commands.entity(cursor_entity).add_child(entity);
    }
}

#[allow(clippy::type_complexity)]
pub fn update_board_binary_displays(
    q_io_pins: Query<
        (&Parent, &SignalState),
        Or<(With<BoardBinaryInputPin>, With<BoardBinaryOutputPin>)>,
    >,
    q_board_io: Query<&Children, Or<(With<BoardBinaryInput>, With<BoardBinaryOutput>)>>,
    q_io_displays: Query<&Children, With<BoardBinaryDisplay>>,
    mut q_io_display_texts: Query<(&mut Text, &Parent)>,
) {
    for (parent, signal_state) in q_io_pins.iter() {
        let board_io_children = q_board_io
            .get(parent.get())
            .expect("BoardBinaryInputPin/BoardBinaryOutputPin has no BoardBinaryInput/BoardBinaryOutput parent.");

        let io_display_entity = board_io_children
            .iter()
            .find(|c| q_io_displays.get(**c).is_ok())
            .expect("BoardBinaryInput/BoardBinaryOutput has no BoardBinaryDisplay child.");

        let mut io_display_text = q_io_display_texts
            .iter_mut()
            .find(|t| t.1.get() == *io_display_entity)
            .expect("BoardBinaryDisplay has no Text child.");

        io_display_text.0.sections[0].value = match signal_state {
            SignalState::High => "1".into(),
            SignalState::Low => "0".into(),
        };
    }
}
