use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use moonshine_core::object::{Object, ObjectInstance};
use moonshine_view::{BuildView, ViewCommands};

use crate::events::events::SpawnBoardEntityEvent;
use crate::get_cursor_mut;
use crate::simulation::expressions::Expr;

use crate::designer::{
    board_entity::BoardEntityView, bounding_box::BoundingBox,
    render_settings::CircuitBoardRenderingSettings, signal_state::SignalState,
};

use super::board_entity::{
    BoardEntityModelBundle, BoardEntityViewBundle, BoardEntityViewKind, Position,
};
use super::cursor::Cursor;
use super::pin::{PinCollection, PinModel, PinModelCollection, PinType, PinView, PinViewBundle};

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct ChipSpec {
    pub name: String,
    //pub expression: Expr,
}

#[derive(Resource)]
pub struct ChipSpecs(pub Vec<ChipSpec>);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Chip;

#[derive(Bundle)]
pub struct ChipBundle {
    chip: Chip,
    chip_spec: ChipSpec,
    model_bundle: BoardEntityModelBundle,
    pin_model_collection: PinModelCollection,
}

impl ChipBundle {
    fn new(chip_spec: ChipSpec, position: Position) -> Self {
        let num_input_pins = 2;
        let mut pin_models: Vec<PinModel> = (0..num_input_pins)
            .map(|i| PinModel {
                label: i.to_string(),
                signal_state: SignalState::Low,
                pin_type: PinType::Input,
            })
            .collect();

        pin_models.push(PinModel {
            label: "Y".into(),
            signal_state: SignalState::Low,
            pin_type: PinType::Output,
        });

        Self {
            chip: Chip,
            chip_spec,
            model_bundle: BoardEntityModelBundle::new(position),
            pin_model_collection: PinModelCollection(pin_models),
        }
    }
}

#[derive(Component)]
pub struct ChipLabel;

#[derive(Bundle)]
pub struct ChipLabelBundle {
    chip_label: ChipLabel,
    text_bundle: Text2dBundle,
}

impl ChipLabelBundle {
    fn new(label: String, text_style: TextStyle) -> Self {
        Self {
            chip_label: ChipLabel,
            text_bundle: Text2dBundle {
                text: Text::from_section(label, text_style).with_justify(JustifyText::Center),
                transform: Transform::from_xyz(0.0, 0.0, 0.01),
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct ChipBody;

#[derive(Bundle)]
pub struct ChipBodyBundle {
    chip_body: ChipBody,
    fill: Fill,
    stroke: Stroke,
    shape_bundle: ShapeBundle,
}

impl ChipBodyBundle {
    fn new(render_settings: &CircuitBoardRenderingSettings, chip_spec: &ChipSpec) -> Self {
        let num_input_pins = 2;

        let chip_extents: Vec2 = Vec2::new(
            render_settings.chip_pin_gap * (num_input_pins + 1) as f32,
            render_settings.chip_pin_gap * (num_input_pins + 1) as f32,
        );

        Self {
            chip_body: ChipBody,
            fill: Fill::color(render_settings.chip_color),
            stroke: Stroke::new(
                render_settings.board_entity_stroke_color,
                render_settings.board_entity_stroke_width,
            ),
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: chip_extents,
                    ..default()
                }),
                spatial: SpatialBundle {
                    transform: Transform::IDENTITY,
                    ..default()
                },
                ..default()
            },
        }
    }
}

//TODO: somehow merge these, they are too similar
#[derive(Component)]
pub struct ChipInputPin;

#[derive(Bundle)]
pub struct ChipInputPinBundle {
    chip_input_pin: ChipInputPin,
    pin_view_bundle: PinViewBundle,
}

impl ChipInputPinBundle {
    fn new(
        render_settings: &CircuitBoardRenderingSettings,
        pin_index: u32,
        translation: Vec3,
    ) -> Self {
        Self {
            chip_input_pin: ChipInputPin,
            pin_view_bundle: PinViewBundle::new(
                render_settings,
                pin_index,
                render_settings.chip_pin_radius,
                translation,
            ),
        }
    }
}

#[derive(Component)]
pub struct ChipOutputPin;

#[derive(Bundle)]
pub struct ChipOutputPinBundle {
    chip_output_pin: ChipOutputPin,
    pin_view_bundle: PinViewBundle,
}

impl ChipOutputPinBundle {
    fn new(
        render_settings: &CircuitBoardRenderingSettings,
        pin_index: u32,
        translation: Vec3,
    ) -> Self {
        Self {
            chip_output_pin: ChipOutputPin,
            pin_view_bundle: PinViewBundle::new(
                render_settings,
                pin_index,
                render_settings.chip_pin_radius,
                translation,
            ),
        }
    }
}

#[derive(Component)]
struct ChipPinCollection;

#[derive(Bundle)]
struct ChipPinCollectionBundle {
    chip_pin_collection: ChipPinCollection,
    pin_collection: PinCollection,
    spatial_bundle: SpatialBundle,
}

impl ChipPinCollectionBundle {
    fn new() -> Self {
        Self {
            chip_pin_collection: ChipPinCollection,
            pin_collection: PinCollection,
            spatial_bundle: SpatialBundle::default(),
        }
    }

    fn spawn_pins(
        pin_collection: &mut ChildBuilder,
        render_settings: &CircuitBoardRenderingSettings,
    ) {
        //pin_collection.spawn(ChipPinBundle::new(render_settings));
    }
}

pub fn spawn_chip(
    mut commands: Commands,
    mut spawn_ev: EventReader<SpawnBoardEntityEvent>,
    chip_specs: Res<ChipSpecs>,
) -> Option<(Entity, SpawnBoardEntityEvent)> {
    for ev in spawn_ev.read() {
        let Some(chip_spec) = chip_specs.0.iter().find(|spec| spec.name == ev.name) else {
            continue;
        };

        let entity = commands
            .spawn(ChipBundle::new(chip_spec.clone(), ev.position.clone()))
            .id();

        return Some((entity, ev.clone()));
    }

    None
}

impl BuildView<BoardEntityViewKind> for Chip {
    fn build(
        world: &World,
        object: Object<BoardEntityViewKind>,
        view: &mut ViewCommands<BoardEntityViewKind>,
    ) {
        let asset_server = world.resource::<AssetServer>();
        let render_settings = world.resource::<CircuitBoardRenderingSettings>();

        let font: Handle<Font> = asset_server.load("fonts/VCR_OSD_MONO.ttf");

        let text_style = TextStyle {
            font_size: 20.0,
            color: Color::BLACK,
            font,
        };

        let position = world.get::<Position>(object.entity()).unwrap();
        let chip_spec = world.get::<ChipSpec>(object.entity()).unwrap();
        let num_input_pins = 2;

        let chip_extents: Vec2 = Vec2::new(
            render_settings.chip_pin_gap * (num_input_pins + 1) as f32,
            render_settings.chip_pin_gap * (num_input_pins + 1) as f32,
        );

        view.insert(BoardEntityViewBundle::new(position.clone(), chip_extents))
            .with_children(|board_entity| {
                board_entity.spawn(ChipLabelBundle::new(chip_spec.name.clone(), text_style));
                board_entity.spawn(ChipBodyBundle::new(render_settings, chip_spec));

                board_entity
                    .spawn(ChipPinCollectionBundle::new())
                    .with_children(|pc| ChipPinCollectionBundle::spawn_pins(pc, render_settings));
            });
    }
}

// pub fn spawn_chip_d(
//     mut spawn_ev: EventReader<SpawnBoardEntityEvent>,
//     mut commands: Commands,
//     chip_specs: Res<ChipSpecs>,
//     asset_server: Res<AssetServer>,
//     render_settings: Res<CircuitBoardRenderingSettings>,
//     mut q_cursor: Query<(Entity, &mut Cursor)>,
// ) {
//     let (cursor_entity, mut cursor) = get_cursor_mut!(q_cursor);

//     for ev in spawn_ev.read() {
//         let Some(chip_spec) = chip_specs.0.iter().find(|spec| spec.name == ev.name) else {
//             continue;
//         };

//         let num_input_pins = chip_spec.expression.1;

//         let chip_extents: Vec2 = Vec2::new(
//             render_settings.chip_pin_gap * (num_input_pins + 1) as f32,
//             render_settings.chip_pin_gap * (num_input_pins + 1) as f32,
//         );

//         let chip_shape = shapes::Rectangle {
//             extents: chip_extents,
//             ..default()
//         };

//         let pin_shape = shapes::Circle {
//             radius: render_settings.chip_pin_radius,
//             ..default()
//         };

//         let font: Handle<Font> = asset_server.load("fonts/VCR_OSD_MONO.ttf");

//         let text_style = TextStyle {
//             font_size: 20.0,
//             color: Color::BLACK,
//             font,
//         };

//         commands
//             .spawn((
//                 ShapeBundle {
//                     path: GeometryBuilder::build_as(&chip_shape),
//                     spatial: SpatialBundle {
//                         transform: Transform::IDENTITY,
//                         ..default()
//                     },
//                     ..default()
//                 },
//                 Fill::color(render_settings.chip_color),
//                 Stroke::new(
//                     render_settings.board_entity_stroke_color,
//                     render_settings.board_entity_stroke_width,
//                 ),
//                 Chip,
//                 ChipExtents(chip_extents),
//                 BoundingBox::rect_new(chip_extents / 2.0, true),
//                 chip_spec.clone(),
//                 BoardEntityView,
//             ))
//             .with_children(|chip| {
//                 //Chip Name
//                 chip.spawn(Text2dBundle {
//                     text: Text::from_section(ev.name.to_uppercase(), text_style)
//                         .with_justify(JustifyText::Center),
//                     transform: Transform::from_xyz(0.0, 0.0, 0.01),
//                     ..default()
//                 });

//                 // Input pins
//                 for i in 0..num_input_pins {
//                     chip.spawn((
//                         ShapeBundle {
//                             path: GeometryBuilder::build_as(&pin_shape),
//                             spatial: SpatialBundle {
//                                 transform: Transform::from_xyz(
//                                     -(chip_extents.x / 2.0),
//                                     (i as f32 * render_settings.chip_pin_gap)
//                                         - (chip_extents.y / 2.0)
//                                         + render_settings.chip_pin_gap,
//                                     0.01,
//                                 ),
//                                 ..default()
//                             },
//                             ..default()
//                         },
//                         Fill::color(render_settings.pin_color),
//                         ChipInputPin,
//                         SignalState::Low,
//                         BoundingBox::circle_new(render_settings.board_binary_io_pin_radius, false),
//                     ));
//                 }

//                 // Output pins
//                 chip.spawn((
//                     ShapeBundle {
//                         path: GeometryBuilder::build_as(&pin_shape),
//                         spatial: SpatialBundle {
//                             transform: Transform::from_xyz(chip_extents.x / 2.0, 0.0, 0.01),
//                             ..default()
//                         },
//                         ..default()
//                     },
//                     Fill::color(render_settings.pin_color),
//                     ChipOutputPin,
//                     SignalState::Low,
//                     BoundingBox::circle_new(render_settings.board_binary_io_pin_radius, false),
//                 ));
//             });
//     }
// }
