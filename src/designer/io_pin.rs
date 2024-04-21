use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{events::events::SpawnBoardEntityEvent, get_cursor, get_cursor_mut};

use super::{
    board_entity::BoardEntity,
    bounding_box::BoundingBox,
    cursor::{Cursor, CursorState},
    render_settings::CircuitBoardRenderingSettings,
    selection::Selected,
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

#[derive(Component)]
pub struct IOPinState {
    pub signal_state: SignalState,
}

pub fn spawn_board_binary_input(
    mut commands: Commands,
    mut spawn_ev: EventReader<SpawnBoardEntityEvent>,
    render_settings: Res<CircuitBoardRenderingSettings>,
    mut q_cursor: Query<(Entity, &mut Cursor)>,
    asset_server: Res<AssetServer>,
) -> Option<(Entity, SpawnBoardEntityEvent)> {
    let (cursor_entity, mut cursor) = get_cursor_mut!(q_cursor);

    for ev in spawn_ev.read() {
        if ev.name != "PORT-IN" {
            continue;
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
            Fill::color(render_settings.pin_color),
            BoundingBox::circle_new(render_settings.binary_io_pin_radius, false),
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
            IOPinState {
                signal_state: SignalState::Low,
            },
            BoundingBox::rect_with_offset(
                binary_input_extents / Vec2::new(4.0, 2.0),
                Vec2::new(binary_input_extents.x / 4.0, 0.0),
                true,
            ),
            Fill::color(render_settings.binary_io_color),
            Stroke::new(
                render_settings.board_entity_stroke_color,
                render_settings.board_entity_stroke_width,
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
        commands.entity(entity).insert(Selected);

        return Some((entity, ev.clone()));
    }

    None
}

pub fn spawn_board_binary_output(
    mut commands: Commands,
    mut spawn_ev: EventReader<SpawnBoardEntityEvent>,
    render_settings: Res<CircuitBoardRenderingSettings>,
    mut q_cursor: Query<(Entity, &mut Cursor)>,
    asset_server: Res<AssetServer>,
) -> Option<(Entity, SpawnBoardEntityEvent)> {
    let (cursor_entity, mut cursor) = get_cursor_mut!(q_cursor);

    for ev in spawn_ev.read() {
        if ev.name != "PORT-OUT" {
            continue;
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
            Fill::color(render_settings.pin_color),
            BoardBinaryOutputPin,
            BoundingBox::circle_new(render_settings.binary_io_pin_radius, false),
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
            IOPinState {
                signal_state: SignalState::Low,
            },
            BoundingBox::rect_new(rect_shape.extents / 2.0, true),
            Stroke::new(
                render_settings.board_entity_stroke_color,
                render_settings.board_entity_stroke_width,
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
        commands.entity(entity).insert(Selected);

        return Some((entity, ev.clone()));
    }

    None
}

#[allow(clippy::type_complexity)]
pub fn apply_io_pin_state(
    q_io_pins: Query<
        (Entity, &IOPinState),
        (
            Or<(With<BoardBinaryInput>, With<BoardBinaryOutput>)>,
            Changed<IOPinState>,
        ),
    >,
    mut q_io_display_texts: Query<(&mut Text, &Parent)>,
) {
    for (io_pin_entity, io_pin_state) in q_io_pins.iter() {
        let mut io_display_text = q_io_display_texts
            .iter_mut()
            .find(|t| **t.1 == io_pin_entity)
            .expect("BoardBinaryInput/BoardBinaryOutput has no Text child.");

        io_display_text.0.sections[0].value = match io_pin_state.signal_state {
            SignalState::High => "1".into(),
            SignalState::Low => "0".into(),
        };
    }
}

pub fn toggle_board_input_switch(
    input: Res<ButtonInput<MouseButton>>,
    mut q_inputs: Query<&mut IOPinState, With<BoardBinaryInput>>,
    q_input_switches: Query<(&Parent, &BoundingBox), With<BoardBinaryInputSwitch>>,
    q_cursor: Query<&Transform, With<Cursor>>,
) {
    let cursor_transform = get_cursor!(q_cursor);

    if input.just_pressed(MouseButton::Left) {
        for (parent, bbox) in q_input_switches.iter() {
            if !bbox.point_in_bbox(cursor_transform.translation.truncate()) {
                continue;
            }

            let mut io_pin_state = q_inputs
                .get_mut(parent.get())
                .expect("BoardBinaryInputSwitch has no BoardBinaryInput parent.");

            io_pin_state.signal_state.toggle();

            break;
        }
    }
}
