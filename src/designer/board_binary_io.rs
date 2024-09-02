use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use moonshine_core::prelude::*;
use moonshine_view::prelude::*;
use uuid::Uuid;

use crate::{events::events::SpawnBoardEntityEvent, find_descendant, get_cursor, get_model_mut};

use super::{
    board_entity::{BoardEntityModelBundle, BoardEntityViewBundle, BoardEntityViewKind, Position},
    bounding_box::BoundingBox,
    cursor::Cursor,
    pin::{PinCollectionBundle, PinModel, PinModelCollection, PinType, PinViewBundle},
    render_settings::CircuitBoardRenderingSettings,
    signal_state::SignalState,
};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BoardBinaryInput;

#[derive(Bundle)]
pub struct BoardBinaryInputBundle {
    board_binary_input: BoardBinaryInput,
    model_bundle: BoardEntityModelBundle,
    pin_model_collection: PinModelCollection,
}

impl BoardBinaryInputBundle {
    fn new(position: Position) -> Self {
        Self {
            board_binary_input: BoardBinaryInput,
            model_bundle: BoardEntityModelBundle::new(position),
            pin_model_collection: PinModelCollection(vec![PinModel {
                label: "".into(),
                pin_type: PinType::Output,
                signal_state: SignalState::Low,
                uuid: Uuid::new_v4(),
            }]),
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
                    -render_settings.board_binary_input_extents.x / 4.0,
                    0.0,
                    0.01,
                ),
                sprite: Sprite {
                    custom_size: Some(
                        render_settings.board_binary_input_extents / Vec2::new(2.0, 1.0) * 0.8,
                    ),
                    ..default()
                },
                ..default()
            },
            bounding_box: BoundingBox::rect_new(
                render_settings.board_binary_input_extents / Vec2::new(4.0, 2.0),
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
        let points = vec![
            Vec2::new(-1.0, -1.0),
            Vec2::new(-1.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, -1.0),
        ]
        .into_iter()
        .map(|x| x * (render_settings.board_binary_input_extents / 2.0))
        .collect();

        Self {
            board_binary_input_body: BoardBinaryInputBody,
            fill: Fill::color(render_settings.board_binary_io_color),
            stroke: Stroke::new(
                render_settings.board_entity_stroke_color,
                render_settings.board_entity_stroke_width,
            ),
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::RoundedPolygon {
                    points,
                    radius: 5.0,
                    closed: false,
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
    pin_view_bundle: PinViewBundle,
}

impl BoardBinaryInputPinBundle {
    fn new(render_settings: &CircuitBoardRenderingSettings, uuid: Uuid) -> Self {
        Self {
            board_binary_input_pin: BoardBinaryInputPin,
            pin_view_bundle: PinViewBundle::new(
                render_settings,
                uuid,
                render_settings.board_binary_io_pin_radius,
                Vec3::new(
                    render_settings.board_binary_input_extents.x / 2.0,
                    0.0,
                    0.02,
                ),
            ),
        }
    }
}

#[derive(Component)]
struct BoardBinaryInputPinCollection;

#[derive(Bundle)]
struct BoardBinaryInputPinCollectionBundle {
    board_binary_input_pin_collection: BoardBinaryInputPinCollection,
    pin_collection_bundle: PinCollectionBundle,
}

impl BoardBinaryInputPinCollectionBundle {
    fn new() -> Self {
        Self {
            board_binary_input_pin_collection: BoardBinaryInputPinCollection,
            pin_collection_bundle: PinCollectionBundle::new(),
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BoardBinaryOutput;

#[derive(Bundle)]
pub struct BoardBinaryOutputBundle {
    board_binary_output: BoardBinaryOutput,
    board_entity_model_bundle: BoardEntityModelBundle,
    pin_model_collection: PinModelCollection,
}

impl BoardBinaryOutputBundle {
    fn new(position: Position) -> Self {
        Self {
            board_binary_output: BoardBinaryOutput,
            board_entity_model_bundle: BoardEntityModelBundle::new(position),
            pin_model_collection: PinModelCollection(vec![PinModel {
                label: "".into(),
                pin_type: PinType::Input,
                signal_state: SignalState::Low,
                uuid: Uuid::new_v4(),
            }]),
        }
    }
}

#[derive(Component)]
pub struct BoardBinaryOutputBody;

#[derive(Bundle)]
pub struct BoardBinaryOutputBodyBundle {
    board_binary_output_body: BoardBinaryOutputBody,
    fill: Fill,
    stroke: Stroke,
    shape_bundle: ShapeBundle,
}

impl BoardBinaryOutputBodyBundle {
    fn new(render_settings: &CircuitBoardRenderingSettings) -> Self {
        let points = vec![
            Vec2::new(-1.0, -1.0),
            Vec2::new(-1.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, -1.0),
        ]
        .into_iter()
        .map(|x| x * (render_settings.board_binary_output_extents / 2.0))
        .collect();

        Self {
            board_binary_output_body: BoardBinaryOutputBody,
            fill: Fill::color(render_settings.board_binary_io_color),
            stroke: Stroke::new(
                render_settings.board_entity_stroke_color,
                render_settings.board_entity_stroke_width,
            ),
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::RoundedPolygon {
                    points,
                    radius: 5.0,
                    closed: false,
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
pub struct BoardBinaryOutputPin;

#[derive(Bundle)]
pub struct BoardBinaryOutputPinBundle {
    board_binary_output_pin: BoardBinaryOutputPin,
    pin_view_bundle: PinViewBundle,
}

impl BoardBinaryOutputPinBundle {
    fn new(render_settings: &CircuitBoardRenderingSettings, uuid: Uuid) -> Self {
        Self {
            board_binary_output_pin: BoardBinaryOutputPin,
            pin_view_bundle: PinViewBundle::new(
                render_settings,
                uuid,
                render_settings.board_binary_io_pin_radius,
                Vec3::new(
                    -render_settings.board_binary_output_extents.x / 2.0,
                    0.0,
                    0.02,
                ),
            ),
        }
    }
}

#[derive(Component)]
struct BoardBinaryOutputPinCollection;

#[derive(Bundle)]
struct BoardBinaryOutputPinCollectionBundle {
    board_binary_output_pin_collection: BoardBinaryOutputPinCollection,
    pin_collection_bundle: PinCollectionBundle,
}

impl BoardBinaryOutputPinCollectionBundle {
    fn new() -> Self {
        Self {
            board_binary_output_pin_collection: BoardBinaryOutputPinCollection,
            pin_collection_bundle: PinCollectionBundle::new(),
        }
    }
}

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
        asset_server: &AssetServer,
        is_input: bool,
    ) -> Self {
        let font: Handle<Font> = asset_server.load("fonts/VCR_OSD_MONO.ttf");

        let text_style = TextStyle {
            font_size: render_settings.board_binary_io_display_font_size,
            color: Color::BLACK,
            font,
        };

        let x_offset = match is_input {
            true => render_settings.board_binary_input_extents.x / 4.0,
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

pub fn spawn_board_binary_input(
    mut commands: Commands,
    mut spawn_ev: EventReader<SpawnBoardEntityEvent>,
) -> Option<(Entity, SpawnBoardEntityEvent)> {
    for ev in spawn_ev.read() {
        if ev.name != "PORT-IN" {
            continue;
        }

        let entity = commands
            .spawn(BoardBinaryInputBundle::new(ev.position.clone()))
            .id();

        return Some((entity, ev.clone()));
    }

    None
}

impl BuildView<BoardEntityViewKind> for BoardBinaryInput {
    fn build(
        world: &World,
        object: Object<BoardEntityViewKind>,
        view: &mut ViewCommands<BoardEntityViewKind>,
    ) {
        let asset_server = world.resource::<AssetServer>();
        let render_settings = world.resource::<CircuitBoardRenderingSettings>();

        let position = world.get::<Position>(object.entity()).unwrap();
        let pin_model_collection = world.get::<PinModelCollection>(object.entity()).unwrap();

        view.insert(BoardEntityViewBundle::new(
            position.clone(),
            render_settings.board_binary_input_extents,
        ))
        .with_children(|board_entity| {
            board_entity.spawn(BoardBinaryInputSwitchBundle::new(
                render_settings,
                asset_server.load("images/switch.png"),
            ));
            board_entity.spawn(BoardBinaryInputBodyBundle::new(render_settings));
            board_entity.spawn(BoardBinaryDisplayBundle::new(
                render_settings,
                asset_server,
                true,
            ));

            board_entity
                .spawn(BoardBinaryInputPinCollectionBundle::new())
                .with_children(|pc| {
                    pc.spawn(BoardBinaryInputPinBundle::new(
                        render_settings,
                        pin_model_collection[0].uuid,
                    ));
                });
        });
    }
}

pub fn spawn_board_binary_output(
    mut commands: Commands,
    mut spawn_ev: EventReader<SpawnBoardEntityEvent>,
) -> Option<(Entity, SpawnBoardEntityEvent)> {
    for ev in spawn_ev.read() {
        if ev.name != "PORT-OUT" {
            continue;
        }

        let entity = commands
            .spawn(BoardBinaryOutputBundle::new(Position::new(0.0, 0.0)))
            .id();

        return Some((entity, ev.clone()));
    }

    None
}

impl BuildView<BoardEntityViewKind> for BoardBinaryOutput {
    fn build(
        world: &World,
        object: Object<BoardEntityViewKind>,
        view: &mut ViewCommands<BoardEntityViewKind>,
    ) {
        let asset_server = world.resource::<AssetServer>();
        let render_settings = world.resource::<CircuitBoardRenderingSettings>();

        let position = world.get::<Position>(object.entity()).unwrap();
        let pin_model_collection = world.get::<PinModelCollection>(object.entity()).unwrap();

        view.insert(BoardEntityViewBundle::new(
            position.clone(),
            render_settings.board_binary_output_extents,
        ))
        .with_children(|board_entity| {
            board_entity.spawn(BoardBinaryOutputBodyBundle::new(render_settings));
            board_entity.spawn(BoardBinaryDisplayBundle::new(
                render_settings,
                asset_server,
                false,
            ));
            board_entity
                .spawn(BoardBinaryOutputPinCollectionBundle::new())
                .with_children(|pc| {
                    pc.spawn(BoardBinaryOutputPinBundle::new(
                        render_settings,
                        pin_model_collection[0].uuid,
                    ));
                });
        });
    }
}

#[allow(clippy::type_complexity)]
pub fn update_board_binary_displays(
    q_board_binary_io: Query<
        (&PinModelCollection, &Viewable<BoardEntityViewKind>),
        (
            Or<(With<BoardBinaryInput>, With<BoardBinaryOutput>)>,
            Changed<PinModelCollection>,
        ),
    >,
    q_children: Query<&Children>,
    mut q_displays: Query<&mut Text, With<BoardBinaryDisplay>>,
) {
    for (pin_model_collection, viewable) in q_board_binary_io.iter() {
        let view_entity = viewable.view().entity();

        find_descendant!(q_children, view_entity, q_displays, |target: &mut Text| {
            target.sections[0].value = match pin_model_collection[0].signal_state {
                SignalState::High => "1".into(),
                SignalState::Low => "0".into(),
            };
        });
    }
}

pub fn toggle_board_input_switch(
    input: Res<ButtonInput<MouseButton>>,
    q_input_switches: Query<(Entity, &BoundingBox), With<BoardBinaryInputSwitch>>,
    q_cursor: Query<&Transform, With<Cursor>>,
    q_parents: Query<&Parent>,
    q_board_entities: Query<&View<BoardEntityViewKind>>,
    mut q_board_binary_input_model: Query<&mut PinModelCollection, With<BoardBinaryInput>>,
) {
    let cursor_transform = get_cursor!(q_cursor);

    if input.just_pressed(MouseButton::Left) {
        for (switch_entity, bbox) in q_input_switches.iter() {
            if !bbox.point_in_bbox(cursor_transform.translation.truncate()) {
                continue;
            }

            let Some(mut pin_collection) = get_model_mut!(
                q_parents,
                q_board_entities,
                q_board_binary_input_model,
                switch_entity
            ) else {
                return;
            };

            pin_collection[0].signal_state.toggle();

            break;
        }
    }
}
