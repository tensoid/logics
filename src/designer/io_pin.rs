use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{events::events::SpawnIOPinEvent, get_cursor_mut};

use super::{
    board_entity::BoardEntity,
    bounding_box::BoundingBox,
    cursor::{Cursor, CursorState},
    draw_layer::DrawLayer,
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

        let font: Handle<Font> = asset_server.load("fonts/VCR_OSD_MONO.ttf");

        let text_style = TextStyle {
            font_size: 20.0,
            color: Color::BLACK,
            font,
        };

        let pin_bundle = (
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: render_settings.binary_io_pin_radius,
                    ..default()
                }),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(
                        render_settings.binary_io_display_extents.x,
                        0.0,
                        DrawLayer::Pin.get_z(),
                    ),
                    ..default()
                },
                ..default()
            },
            Fill::color(render_settings.signal_low_color),
        );

        //TODO: fix z's

        let board_binary_display_shape = ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                extents: render_settings.binary_io_display_extents,
                ..default()
            }),
            spatial: SpatialBundle {
                transform: Transform::from_xyz(
                    render_settings.binary_io_display_extents.x / 2.0,
                    0.0,
                    0.0,
                ),
                ..default()
            },
            ..default()
        };

        let board_binary_display_text = Text2dBundle {
            text: Text::from_section("0", text_style).with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 0.0, DrawLayer::ChipName.get_z()),
            ..default()
        };

        let board_binary_display_bundle = (
            BoardBinaryDisplay,
            board_binary_display_shape,
            Fill::color(render_settings.binary_io_display_color),
        );

        let io_spatial_bundle = SpatialBundle {
            transform: Transform::from_xyz(ev.position.x, ev.position.y, DrawLayer::Pin.get_z()),
            ..default()
        };

        let board_binary_switch_shape = ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                extents: render_settings.binary_io_switch_extents,
                ..default()
            }),
            spatial: SpatialBundle {
                transform: Transform::from_xyz(
                    -render_settings.binary_io_switch_extents.x / 2.0,
                    0.0,
                    0.0,
                ),
                ..default()
            },
            ..default()
        };

        let board_binary_switch_bundle = (
            BoardBinaryInputSwitch,
            board_binary_switch_shape,
            Fill::color(render_settings.binary_io_switch_color),
            BoundingBox::new(render_settings.binary_io_switch_extents / 2.0, false),
        );

        let board_binary_switch_sprite_bundle = SpriteBundle {
            texture: asset_server.load("images/switch.png"),
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            sprite: Sprite {
                custom_size: Some(render_settings.binary_io_switch_extents * 0.8),
                ..default()
            },
            ..default()
        };

        let board_binary_input_bundle = (
            BoardBinaryInput,
            io_spatial_bundle,
            BoardEntity,
            BoundingBox::with_offset(
                render_settings.binary_io_display_extents / 2.0,
                Vec2::new(render_settings.binary_io_display_extents.x / 2.0, 0.0),
                true,
            ),
        );

        let entity = commands
            .spawn(board_binary_input_bundle)
            .with_children(|bbi| {
                bbi.spawn(board_binary_switch_bundle).with_children(|bbis| {
                    bbis.spawn(board_binary_switch_sprite_bundle);
                });
                bbi.spawn(board_binary_display_bundle)
                    .with_children(|bbid| {
                        bbid.spawn(board_binary_display_text);
                    });
                bbi.spawn((pin_bundle, BoardBinaryInputPin, SignalState::Low));
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

        let rect_shape = shapes::Rectangle {
            extents: render_settings.binary_io_display_extents,
            ..default()
        };

        let pin_shape = shapes::Circle {
            radius: render_settings.binary_io_pin_radius,
            ..default()
        };

        let pin_bundle = (
            ShapeBundle {
                path: GeometryBuilder::build_as(&pin_shape),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(
                        -rect_shape.extents.x / 2.0,
                        0.0,
                        DrawLayer::Pin.get_z(),
                    ),
                    ..default()
                },
                ..default()
            },
            Fill::color(render_settings.signal_low_color),
        );

        let font: Handle<Font> = asset_server.load("fonts/VCR_OSD_MONO.ttf");

        let text_style = TextStyle {
            font_size: 20.0,
            color: Color::BLACK,
            font,
        };

        let board_binary_display_shape = ShapeBundle {
            path: GeometryBuilder::build_as(&rect_shape),
            ..default()
        };

        let board_binary_display_text = Text2dBundle {
            text: Text::from_section("0", text_style).with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 0.0, DrawLayer::ChipName.get_z()),
            ..default()
        };

        let board_binary_display_bundle = (
            BoardBinaryDisplay,
            board_binary_display_shape,
            Fill::color(render_settings.binary_io_display_color),
        );

        let io_spatial_bundle = SpatialBundle {
            transform: Transform::from_xyz(ev.position.x, ev.position.y, DrawLayer::Pin.get_z()),
            ..default()
        };

        let board_binary_output_bundle = (
            BoardBinaryOutput,
            io_spatial_bundle,
            BoardEntity,
            BoundingBox::new(rect_shape.extents / 2.0, true),
        );

        let entity = commands
            .spawn(board_binary_output_bundle)
            .with_children(|bbo| {
                bbo.spawn(board_binary_display_bundle)
                    .with_children(|bbod| {
                        bbod.spawn(board_binary_display_text);
                    });
                bbo.spawn((pin_bundle, BoardBinaryOutputPin, SignalState::Low));
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

    /*
    let parent_children = q_inputs
        .get(parent.get())
        .expect("BoardBinaryInputSwitch has no BoardBinaryInput parent.");

    let board_binary_input_pin_entity = parent_children
        .iter()
        .find(|c| q_input_pins.get(**c).is_ok())
        .expect("BoardBinaryInput has no BoardBinaryInputPin child.");

    let (_, mut board_binary_input_pin_state) = q_input_pins
        .get_mut(*board_binary_input_pin_entity)
        .expect("BoardBinaryInput has no BoardBinaryInputPin child.");
    */
}
