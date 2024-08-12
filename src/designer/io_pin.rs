use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use moonshine_core::prelude::*;
use moonshine_view::prelude::*;

use crate::{events::events::SpawnBoardEntityEvent, get_cursor, get_cursor_mut};

use super::{
    board_entity::{BoardEntityModelBundle, BoardEntityView, BoardEntityViewBundle, Position},
    bounding_box::BoundingBox,
    cursor::{Cursor, CursorState},
    render_settings::CircuitBoardRenderingSettings,
    selection::Selected,
    signal_state::SignalState,
};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BoardBinaryInput;

#[derive(Bundle)]
pub struct BoardBinaryInputBundle {
    board_binary_input: BoardBinaryInput,
    board_entity_model_bundle: BoardEntityModelBundle,
}

impl BoardBinaryInputBundle {
    fn new(position: Position) -> Self {
        Self {
            board_binary_input: BoardBinaryInput,
            board_entity_model_bundle: BoardEntityModelBundle::new(position),
        }
    }
}

#[derive(Component)]
pub struct BoardBinaryInputSwitch;

#[derive(Bundle)]
pub struct BoardBinaryInputSwitchBundle {
    board_binary_input_switch: BoardBinaryInputSwitch,
    sprite_bundle: SpriteBundle,
    bounding_box: BoundingBox,
}

impl BoardBinaryInputSwitchBundle {
    fn new(render_settings: &CircuitBoardRenderingSettings, texture: Handle<Image>) -> Self {
        Self {
            board_binary_input_switch: BoardBinaryInputSwitch,
            sprite_bundle: SpriteBundle {
                texture,
                transform: Transform::from_xyz(
                    -render_settings.binary_io_extents.x / 4.0,
                    0.0,
                    0.01,
                ),
                sprite: Sprite {
                    custom_size: Some(
                        render_settings.binary_io_extents / Vec2::new(2.0, 1.0) * 0.8,
                    ),
                    ..default()
                },
                ..default()
            },
            bounding_box: BoundingBox::rect_new(
                render_settings.binary_io_extents / Vec2::new(4.0, 2.0),
                false,
            ),
        }
    }
}

#[derive(Component)]
pub struct BoardBinaryInputBody;

#[derive(Bundle)]
pub struct BoardBinaryInputBodyBundle {
    board_binary_input_body: BoardBinaryInputBody,
    fill: Fill,
    stroke: Stroke,
    shape_bundle: ShapeBundle,
}

impl BoardBinaryInputBodyBundle {
    fn new(render_settings: &CircuitBoardRenderingSettings) -> Self {
        Self {
            board_binary_input_body: BoardBinaryInputBody,
            fill: Fill::color(render_settings.binary_io_color),
            stroke: Stroke::new(
                render_settings.board_entity_stroke_color,
                render_settings.board_entity_stroke_width,
            ),
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: render_settings.binary_io_extents,
                    ..default()
                }),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct BoardBinaryInputPin;

#[derive(Bundle)]
pub struct BoardBinaryInputPinBundle {
    board_binary_input_pin: BoardBinaryInputPin,
    shape_bundle: ShapeBundle,
    fill: Fill,
    bounding_box: BoundingBox,
}

impl BoardBinaryInputPinBundle {
    fn new(render_settings: &CircuitBoardRenderingSettings) -> Self {
        Self {
            board_binary_input_pin: BoardBinaryInputPin,
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: render_settings.binary_io_pin_radius,
                    ..default()
                }),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(
                        render_settings.binary_io_extents.x / 2.0,
                        0.0,
                        0.02,
                    ),
                    ..default()
                },
                ..default()
            },
            fill: Fill::color(render_settings.pin_color),
            bounding_box: BoundingBox::circle_new(render_settings.binary_io_pin_radius, false),
        }
    }
}

#[derive(Component)]
pub struct BoardBinaryOutput;

#[derive(Component)]
pub struct BoardBinaryOutputPin;

#[derive(Component)]
pub struct BoardBinaryDisplay;

#[derive(Bundle)]
pub struct BoardBinaryDisplayBundle {
    board_binary_display: BoardBinaryDisplay,
    text_bundle: Text2dBundle,
}

impl BoardBinaryDisplayBundle {
    fn new(
        render_settings: &CircuitBoardRenderingSettings,
        text_style: TextStyle,
        is_input: bool,
    ) -> Self {
        let x_offset = match is_input {
            true => render_settings.binary_io_extents.x / 4.0,
            false => 0.0,
        };

        Self {
            board_binary_display: BoardBinaryDisplay,
            text_bundle: Text2dBundle {
                text: Text::from_section("0", text_style).with_justify(JustifyText::Center),
                transform: Transform::from_xyz(x_offset, 0.0, 0.01),
                ..default()
            },
        }
    }
}

impl BuildView for BoardBinaryInput {
    fn build(world: &World, object: Object<Self>, view: &mut ViewCommands<Self>) {
        let asset_server = world.resource::<AssetServer>();
        let render_settings = world.resource::<CircuitBoardRenderingSettings>();

        //TODO: store textstyle somewhere else like render settings
        let font: Handle<Font> = asset_server.load("fonts/VCR_OSD_MONO.ttf");

        let text_style = TextStyle {
            font_size: 20.0,
            color: Color::BLACK,
            font,
        };

        let position = world.get::<Position>(object.entity()).unwrap();

        view.insert(BoardEntityViewBundle::new(
            position.clone(),
            render_settings.binary_io_extents,
        ))
        .with_children(|board_entity| {
            board_entity.spawn(BoardBinaryInputSwitchBundle::new(
                render_settings,
                asset_server.load("images/switch.png"),
            ));
            board_entity.spawn(BoardBinaryInputBodyBundle::new(render_settings));
            board_entity.spawn(BoardBinaryDisplayBundle::new(
                render_settings,
                text_style,
                true,
            ));
            board_entity.spawn(BoardBinaryInputPinBundle::new(render_settings));
        });
    }
}

pub fn spawn_board_binary_input(
    mut commands: Commands,
    mut spawn_ev: EventReader<SpawnBoardEntityEvent>,
) -> Option<(Entity, SpawnBoardEntityEvent)> {
    for ev in spawn_ev.read() {
        if ev.name != "PORT-IN" {
            continue;
        }

        commands.spawn(BoardBinaryInputBundle::new(Position::new(0.0, 0.0)));
    }

    None
}

pub fn spawn_board_binary_input_d(
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
            SignalState::Low,
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
            BoardEntityView,
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
        // cursor.state = CursorState::Dragging;
        // commands.entity(cursor_entity).add_child(entity);
        // commands.entity(entity).insert(Selected);

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
            SignalState::Low,
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
            BoardEntityView,
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
pub fn update_board_binary_displays(
    q_io_pins: Query<
        (&Parent, &SignalState),
        (
            Or<(With<BoardBinaryInputPin>, With<BoardBinaryOutputPin>)>,
            Changed<SignalState>,
        ),
    >,
    mut q_io_display_texts: Query<(&mut Text, &Parent)>,
) {
    for (parent, signal_state) in q_io_pins.iter() {
        let mut io_display_text = q_io_display_texts
            .iter_mut()
            .find(|t| t.1 == parent)
            .expect("BoardBinaryInput/BoardBinaryOutput has no Text child.");

        io_display_text.0.sections[0].value = match signal_state {
            SignalState::High => "1".into(),
            SignalState::Low => "0".into(),
        };
    }
}

pub fn toggle_board_input_switch(
    input: Res<ButtonInput<MouseButton>>,
    q_inputs: Query<&Children, With<BoardBinaryInput>>,
    q_input_switches: Query<(&Parent, &BoundingBox), With<BoardBinaryInputSwitch>>,
    mut q_input_pins: Query<(&mut BoardBinaryInputPin, &mut SignalState)>,
    q_cursor: Query<&Transform, With<Cursor>>,
) {
    let cursor_transform = get_cursor!(q_cursor);

    if input.just_pressed(MouseButton::Left) {
        for (parent, bbox) in q_input_switches.iter() {
            if !bbox.point_in_bbox(cursor_transform.translation.truncate()) {
                continue;
            }

            //TODO: find a way to make this easier (Child -> Parent -> Children -> Specific Children)
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

            board_binary_input_pin_state.toggle();
            break;
        }
    }
}
