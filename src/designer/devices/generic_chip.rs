use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use moonshine_core::object::{Object, ObjectInstance};
use moonshine_view::{BuildView, ViewCommands};
use uuid::Uuid;

use crate::designer::{
    designer_assets::DesignerAssets,
    pin::{PinCollectionBundle, PinLabelBundle, PinModelCollection, PinViewBundle},
    position::Position,
    render_settings::CircuitBoardRenderingSettings,
};

use super::device::{DeviceModelBundle, DeviceViewBundle, DeviceViewKind};

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct GenericChip {
    pub name: String,
}

#[derive(Bundle, Clone)]
pub struct GenericChipBundle {
    chip: GenericChip,
    pin_model_collection: PinModelCollection,
    model_bundle: DeviceModelBundle,
}

impl GenericChipBundle {
    pub fn new(position: Position, pin_model_collection: PinModelCollection, name: String) -> Self {
        Self {
            chip: GenericChip { name },
            pin_model_collection,
            model_bundle: DeviceModelBundle::new(position),
        }
    }
}

#[derive(Component)]
pub struct GenericChipLabel;

#[derive(Bundle)]
pub struct GenericChipLabelBundle {
    chip_label: GenericChipLabel,
    text_bundle: Text2dBundle,
}

impl GenericChipLabelBundle {
    fn new(label: String, text_style: TextStyle) -> Self {
        Self {
            chip_label: GenericChipLabel,
            text_bundle: Text2dBundle {
                text: Text::from_section(label, text_style).with_justify(JustifyText::Center),
                transform: Transform::from_xyz(0.0, 0.0, 0.01),
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct GenericChipBody;

#[derive(Bundle)]
pub struct GenericChipBodyBundle {
    chip_body: GenericChipBody,
    fill: Fill,
    stroke: Stroke,
    shape_bundle: ShapeBundle,
}

impl GenericChipBodyBundle {
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
            chip_body: GenericChipBody,
            fill: Fill::color(render_settings.chip_color),
            stroke: Stroke::new(
                render_settings.device_stroke_color,
                render_settings.device_stroke_width,
            ),
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::RoundedPolygon {
                    points,
                    radius: render_settings.device_edge_radius,
                    closed: false,
                }),
                spatial: SpatialBundle::default(),
                ..default()
            },
        }
    }
}

//TODO: somehow merge these, they are too similar
#[derive(Component)]
pub struct GenericChipInputPin;

#[derive(Bundle)]
pub struct GenericChipInputPinBundle {
    chip_input_pin: GenericChipInputPin,
    pin_view_bundle: PinViewBundle,
}

impl GenericChipInputPinBundle {
    fn new(render_settings: &CircuitBoardRenderingSettings, uuid: Uuid, translation: Vec3) -> Self {
        Self {
            chip_input_pin: GenericChipInputPin,
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
pub struct GenericChipOutputPin;

#[derive(Bundle)]
pub struct GenericChipOutputPinBundle {
    chip_output_pin: GenericChipOutputPin,
    pin_view_bundle: PinViewBundle,
}

impl GenericChipOutputPinBundle {
    fn new(render_settings: &CircuitBoardRenderingSettings, uuid: Uuid, translation: Vec3) -> Self {
        Self {
            chip_output_pin: GenericChipOutputPin,
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
struct GenericChipPinCollection;

#[derive(Bundle)]
struct GenericChipPinCollectionBundle {
    chip_pin_collection: GenericChipPinCollection,
    pin_collection_bundle: PinCollectionBundle,
}

impl GenericChipPinCollectionBundle {
    fn new() -> Self {
        Self {
            chip_pin_collection: GenericChipPinCollection,
            pin_collection_bundle: PinCollectionBundle::new(),
        }
    }

    fn spawn_pins(
        pin_collection: &mut ChildBuilder,
        render_settings: &CircuitBoardRenderingSettings,
        chip_extents: Vec2,
        pin_model_collection: &PinModelCollection,
        pin_label_text_style: TextStyle,
    ) {
        //Input pins
        for (i, pin_model) in pin_model_collection.iter_inputs().enumerate() {
            pin_collection
                .spawn(GenericChipInputPinBundle::new(
                    render_settings,
                    pin_model.uuid,
                    Vec3::new(
                        -(chip_extents.x / 2.0),
                        ((i as f32 + 0.75) * render_settings.chip_pin_gap) - (chip_extents.y / 2.0),
                        0.01,
                    ),
                ))
                .with_children(|pc| {
                    pc.spawn(PinLabelBundle::new(
                        pin_model.label.clone(),
                        pin_label_text_style.clone(),
                        Vec3::new(12.0, 0.0, 0.2),
                    ));
                });
        }

        // Output pins
        let output_pin_model = pin_model_collection.iter_outputs().next().unwrap(); //TODO: only works with one output chips
        pin_collection
            .spawn(GenericChipOutputPinBundle::new(
                render_settings,
                output_pin_model.uuid,
                Vec3::new(chip_extents.x / 2.0, 0.0, 0.01),
            ))
            .with_children(|pc| {
                pc.spawn(PinLabelBundle::new(
                    output_pin_model.label.clone(),
                    pin_label_text_style,
                    Vec3::new(-12.0, 0.0, 0.2),
                ));
            });
    }
}

impl BuildView<DeviceViewKind> for GenericChip {
    fn build(
        world: &World,
        object: Object<DeviceViewKind>,
        view: &mut ViewCommands<DeviceViewKind>,
    ) {
        let designer_assets = world.resource::<DesignerAssets>();
        let render_settings = world.resource::<CircuitBoardRenderingSettings>();

        let chip_label_text_style = TextStyle {
            font_size: render_settings.chip_label_font_size,
            color: Color::BLACK,
            font: designer_assets.font.clone(),
        };

        let pin_label_text_style = TextStyle {
            font_size: render_settings.chip_pin_label_font_size,
            color: Color::BLACK,
            font: designer_assets.font.clone(),
        };

        let position = world.get::<Position>(object.entity()).unwrap();
        let pin_model_collection = world.get::<PinModelCollection>(object.entity()).unwrap();
        let generic_chip = world.get::<GenericChip>(object.entity()).unwrap();

        let chip_extents = calculate_chip_extents(
            render_settings,
            pin_model_collection.num_inputs(),
            pin_model_collection.num_outputs(),
        );

        view.insert(DeviceViewBundle::new(position.clone(), chip_extents))
            .with_children(|device| {
                device.spawn(GenericChipLabelBundle::new(
                    generic_chip.name.clone(),
                    chip_label_text_style,
                ));
                device.spawn(GenericChipBodyBundle::new(
                    render_settings,
                    pin_model_collection,
                ));

                device
                    .spawn(GenericChipPinCollectionBundle::new())
                    .with_children(|pc| {
                        GenericChipPinCollectionBundle::spawn_pins(
                            pc,
                            render_settings,
                            chip_extents,
                            pin_model_collection,
                            pin_label_text_style,
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
        render_settings.chip_pin_gap * (max as f32 + 1.5),
        render_settings.chip_pin_gap * (max as f32 + 0.5),
    )
}
