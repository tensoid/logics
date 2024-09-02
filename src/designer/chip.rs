use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use moonshine_core::object::{Object, ObjectInstance};
use moonshine_view::{BuildView, ViewCommands};
use uuid::Uuid;

use crate::events::events::SpawnBoardEntityEvent;

use crate::designer::render_settings::CircuitBoardRenderingSettings;

use super::board_entity::{
    BoardEntityModelBundle, BoardEntityViewBundle, BoardEntityViewKind, Position,
};
use super::pin::{PinCollectionBundle, PinModelCollection, PinViewBundle};

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct BuiltinChip {
    pub name: String,
}

#[derive(Bundle, Clone)]
pub struct BuiltinChipBundle {
    pub builtin_chip: BuiltinChip,
    pub pin_model_collection: PinModelCollection,
}

impl BuiltinChipBundle {
    pub fn new(name: String, pin_model_collection: PinModelCollection) -> Self {
        Self {
            builtin_chip: BuiltinChip { name },
            pin_model_collection,
        }
    }
}

#[derive(Resource)]
pub struct BuiltinChips(pub Vec<BuiltinChipBundle>);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Chip;

#[derive(Bundle)]
pub struct ChipBundle {
    chip: Chip,
    builtin_chip_bundle: BuiltinChipBundle,
    model_bundle: BoardEntityModelBundle,
}

impl ChipBundle {
    fn new(position: Position, builtin_chip_bundle: BuiltinChipBundle) -> Self {
        Self {
            chip: Chip,
            builtin_chip_bundle,
            model_bundle: BoardEntityModelBundle::new(position),
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
    fn new(
        render_settings: &CircuitBoardRenderingSettings,
        pin_model_collection: &PinModelCollection,
    ) -> Self {
        let chip_extents = calculate_chip_extents(
            render_settings,
            pin_model_collection.num_inputs(),
            pin_model_collection.num_outputs(),
        );

        let points = vec![
            Vec2::new(-1.0, -1.0),
            Vec2::new(-1.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, -1.0),
        ]
        .into_iter()
        .map(|x| x * (chip_extents / 2.0))
        .collect();

        Self {
            chip_body: ChipBody,
            fill: Fill::color(render_settings.chip_color),
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
    fn new(render_settings: &CircuitBoardRenderingSettings, uuid: Uuid, translation: Vec3) -> Self {
        Self {
            chip_input_pin: ChipInputPin,
            pin_view_bundle: PinViewBundle::new(
                render_settings,
                uuid,
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
    fn new(render_settings: &CircuitBoardRenderingSettings, uuid: Uuid, translation: Vec3) -> Self {
        Self {
            chip_output_pin: ChipOutputPin,
            pin_view_bundle: PinViewBundle::new(
                render_settings,
                uuid,
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
    pin_collection_bundle: PinCollectionBundle,
}

impl ChipPinCollectionBundle {
    fn new() -> Self {
        Self {
            chip_pin_collection: ChipPinCollection,
            pin_collection_bundle: PinCollectionBundle::new(),
        }
    }

    fn spawn_pins(
        pin_collection: &mut ChildBuilder,
        render_settings: &CircuitBoardRenderingSettings,
        chip_extents: Vec2,
        pin_model_collection: &PinModelCollection,
    ) {
        //Input pins
        for (i, pin_model) in pin_model_collection.iter_inputs().enumerate() {
            pin_collection.spawn(ChipInputPinBundle::new(
                render_settings,
                pin_model.uuid,
                Vec3::new(
                    -(chip_extents.x / 2.0),
                    ((i as f32 + 0.75) * render_settings.chip_pin_gap) - (chip_extents.y / 2.0),
                    0.01,
                ),
            ));
        }

        // Output pins
        pin_collection.spawn(ChipOutputPinBundle::new(
            render_settings,
            pin_model_collection.iter_outputs().next().unwrap().uuid, //TODO: only works with one output chips
            Vec3::new(chip_extents.x / 2.0, 0.0, 0.01),
        ));
    }
}

pub fn spawn_chip(
    mut commands: Commands,
    mut spawn_ev: EventReader<SpawnBoardEntityEvent>,
    builtin_chips: Res<BuiltinChips>,
) -> Option<(Entity, SpawnBoardEntityEvent)> {
    for ev in spawn_ev.read() {
        let Some(builtin_chip_blueprint) = builtin_chips
            .0
            .iter()
            .find(|chip| chip.builtin_chip.name == ev.name)
        else {
            continue;
        };

        let mut builtin_chip_bundle = builtin_chip_blueprint.clone();
        builtin_chip_bundle
            .pin_model_collection
            .randomize_pin_uuids();

        let entity = commands
            .spawn(ChipBundle::new(ev.position.clone(), builtin_chip_bundle))
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
            font_size: render_settings.chip_label_font_size,
            color: Color::BLACK,
            font,
        };

        let position = world.get::<Position>(object.entity()).unwrap();
        let pin_model_collection = world.get::<PinModelCollection>(object.entity()).unwrap();
        let builtin_chip = world.get::<BuiltinChip>(object.entity()).unwrap();

        let chip_extents = calculate_chip_extents(
            render_settings,
            pin_model_collection.num_inputs(),
            pin_model_collection.num_outputs(),
        );

        view.insert(BoardEntityViewBundle::new(position.clone(), chip_extents))
            .with_children(|board_entity| {
                board_entity.spawn(ChipLabelBundle::new(builtin_chip.name.clone(), text_style));
                board_entity.spawn(ChipBodyBundle::new(render_settings, pin_model_collection));

                board_entity
                    .spawn(ChipPinCollectionBundle::new())
                    .with_children(|pc| {
                        ChipPinCollectionBundle::spawn_pins(
                            pc,
                            render_settings,
                            chip_extents,
                            pin_model_collection,
                        );
                    });
            });
    }
}

/// Calculates the chip extents based on the amount of input/output pins.
fn calculate_chip_extents(
    render_settings: &CircuitBoardRenderingSettings,
    num_inputs: usize,
    num_outputs: usize,
) -> Vec2 {
    let max = num_inputs.max(num_outputs);

    Vec2::new(
        render_settings.chip_pin_gap * (max as f32 + 1.0),
        render_settings.chip_pin_gap * (max as f32 + 0.5),
    )
}
