use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use moonshine_core::prelude::*;
use moonshine_view::prelude::*;
use uuid::Uuid;

use crate::{
    assets::{common_assets::CommonAssets, designer_assets::DesignerAssets},
    designer::{
        bounding_box::BoundingBox,
        cursor::Cursor,
        pin::{PinModel, PinModelCollection, PinViewBundle, PinViewCollectionBundle},
        position::Position,
        render_settings::CircuitBoardRenderingSettings,
        rotation::{self, Rotation},
        signal::Signal,
    },
    find_descendant, get_cursor, get_model_mut,
};

use super::device::{Device, DeviceModelBundle, DeviceViewBundle, DeviceViewKind};

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct BinarySwitch;

impl Device for BinarySwitch {
    fn create_bundle(position: Position) -> impl Bundle {
        BinarySwitchBundle::new(position)
    }

    fn device_id() -> &'static str {
        "SWITCH"
    }
}

#[derive(Bundle, Clone)]
pub struct BinarySwitchBundle {
    binary_switch: BinarySwitch,
    device_model_bundle: DeviceModelBundle,
    pin_model_collection: PinModelCollection,
}

impl BinarySwitchBundle {
    fn new(position: Position) -> Self {
        Self {
            binary_switch: BinarySwitch,
            device_model_bundle: DeviceModelBundle::new(position),
            pin_model_collection: PinModelCollection(vec![PinModel::new_output("Q".into())]),
        }
    }
}

#[derive(Component)]
pub struct BinarySwitchButton;

#[derive(Bundle)]
pub struct BinarySwitchButtonBundle {
    binary_switch_switch: BinarySwitchButton,
    sprite: Sprite,
    bounding_box: BoundingBox,
    transform: Transform,
}

impl BinarySwitchButtonBundle {
    fn new(render_settings: &CircuitBoardRenderingSettings, texture: Handle<Image>) -> Self {
        Self {
            binary_switch_switch: BinarySwitchButton,
            sprite: Sprite {
                custom_size: Some(
                    render_settings.binary_switch_extents / Vec2::new(2.0, 1.0) * 0.8,
                ),
                image: texture,
                ..default()
            },
            transform: Transform::from_xyz(
                -render_settings.binary_switch_extents.x / 4.0,
                0.0,
                0.01,
            ),
            bounding_box: BoundingBox::rect_new(
                render_settings.binary_switch_extents / Vec2::new(4.0, 2.0),
                false,
            ),
        }
    }
}

#[derive(Component)]
pub struct BinarySwitchBody;

#[derive(Bundle)]
pub struct BinarySwitchBodyBundle {
    binary_switch_body: BinarySwitchBody,
    fill: Fill,
    stroke: Stroke,
    shape_bundle: ShapeBundle,
}

impl BinarySwitchBodyBundle {
    fn new(render_settings: &CircuitBoardRenderingSettings) -> Self {
        Self {
            binary_switch_body: BinarySwitchBody,
            fill: Fill::color(render_settings.binary_io_color),
            stroke: Stroke::new(
                render_settings.device_stroke_color,
                render_settings.device_stroke_width,
            ),
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: render_settings.binary_switch_extents,
                    radii: Some(BorderRadii::single(render_settings.device_border_radius)),
                    ..default()
                }),
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct BinarySwitchPin;

#[derive(Bundle)]
pub struct BinarySwitchPinBundle {
    binary_switch_pin: BinarySwitchPin,
    pin_view_bundle: PinViewBundle,
}

impl BinarySwitchPinBundle {
    fn new(render_settings: &CircuitBoardRenderingSettings, uuid: Uuid) -> Self {
        Self {
            binary_switch_pin: BinarySwitchPin,
            pin_view_bundle: PinViewBundle::new(
                render_settings,
                uuid,
                render_settings.device_io_pin_radius,
                Vec3::new(render_settings.binary_switch_extents.x / 2.0, 0.0, 0.02),
            ),
        }
    }
}

#[derive(Component)]
struct BinarySwitchPinCollection;

#[derive(Bundle)]
struct BinarySwitchPinCollectionBundle {
    binary_switch_pin_collection: BinarySwitchPinCollection,
    pin_collection_bundle: PinViewCollectionBundle,
}

impl BinarySwitchPinCollectionBundle {
    fn new() -> Self {
        Self {
            binary_switch_pin_collection: BinarySwitchPinCollection,
            pin_collection_bundle: PinViewCollectionBundle::new(),
        }
    }
}

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct BinaryDisplay;

impl Device for BinaryDisplay {
    fn create_bundle(position: Position) -> impl Bundle {
        BinaryDisplayBundle::new(position)
    }

    fn device_id() -> &'static str {
        "DISPLAY"
    }
}

#[derive(Bundle, Clone)]
pub struct BinaryDisplayBundle {
    binary_display: BinaryDisplay,
    device_model_bundle: DeviceModelBundle,
    pin_model_collection: PinModelCollection,
}

impl BinaryDisplayBundle {
    fn new(position: Position) -> Self {
        Self {
            binary_display: BinaryDisplay,
            device_model_bundle: DeviceModelBundle::new(position),
            pin_model_collection: PinModelCollection(vec![PinModel::new_input("Q".into())]),
        }
    }
}

#[derive(Component)]
pub struct BinaryDisplayBody;

#[derive(Bundle)]
pub struct BinaryDisplayBodyBundle {
    binary_display_body: BinaryDisplayBody,
    fill: Fill,
    stroke: Stroke,
    shape_bundle: ShapeBundle,
}

impl BinaryDisplayBodyBundle {
    fn new(render_settings: &CircuitBoardRenderingSettings) -> Self {
        Self {
            binary_display_body: BinaryDisplayBody,
            fill: Fill::color(render_settings.binary_io_color),
            stroke: Stroke::new(
                render_settings.device_stroke_color,
                render_settings.device_stroke_width,
            ),
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: render_settings.binary_display_extents,
                    radii: Some(BorderRadii::single(render_settings.device_border_radius)),
                    ..default()
                }),
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct BinaryDisplayPin;

#[derive(Bundle)]
pub struct BinaryDisplayPinBundle {
    binary_display_pin: BinaryDisplayPin,
    pin_view_bundle: PinViewBundle,
}

impl BinaryDisplayPinBundle {
    fn new(render_settings: &CircuitBoardRenderingSettings, uuid: Uuid) -> Self {
        Self {
            binary_display_pin: BinaryDisplayPin,
            pin_view_bundle: PinViewBundle::new(
                render_settings,
                uuid,
                render_settings.device_io_pin_radius,
                Vec3::new(-render_settings.binary_display_extents.x / 2.0, 0.0, 0.02),
            ),
        }
    }
}

#[derive(Component)]
struct BinaryDisplayPinCollection;

#[derive(Bundle)]
struct BinaryDisplayPinCollectionBundle {
    binary_display_pin_collection: BinaryDisplayPinCollection,
    pin_collection_bundle: PinViewCollectionBundle,
}

impl BinaryDisplayPinCollectionBundle {
    fn new() -> Self {
        Self {
            binary_display_pin_collection: BinaryDisplayPinCollection,
            pin_collection_bundle: PinViewCollectionBundle::new(),
        }
    }
}

#[derive(Component)]
pub struct BoardBinaryDisplay;

#[derive(Bundle)]
pub struct BoardBinaryDisplayBundle {
    board_binary_display: BoardBinaryDisplay,
    text_2d: Text2d,
    text_color: TextColor,
    text_font: TextFont,
    text_layout: TextLayout,
    transform: Transform,
}

impl BoardBinaryDisplayBundle {
    fn new(
        render_settings: &CircuitBoardRenderingSettings,
        common_assets: &CommonAssets,
        is_input: bool,
    ) -> Self {
        let x_offset = match is_input {
            true => render_settings.binary_switch_extents.x / 4.0,
            false => 0.0,
        };

        Self {
            board_binary_display: BoardBinaryDisplay,
            text_2d: Text2d::new("0"),
            text_color: TextColor(Color::BLACK),
            text_font: TextFont {
                font_size: render_settings.binary_display_font_size,
                font: common_assets.font.clone(),
                ..default()
            },
            text_layout: TextLayout::new_with_justify(JustifyText::Center),
            transform: Transform::from_xyz(x_offset, 0.0, 0.01),
        }
    }
}

impl BuildView<DeviceViewKind> for BinarySwitch {
    fn build(
        world: &World,
        object: Object<DeviceViewKind>,
        mut view: ViewCommands<DeviceViewKind>,
    ) {
        let common_assets = world.resource::<CommonAssets>();
        let designer_assets = world.resource::<DesignerAssets>();
        let render_settings = world.resource::<CircuitBoardRenderingSettings>();

        let position = world.get::<Position>(object.entity()).unwrap();
        let rotation = world.get::<Rotation>(object.entity()).unwrap();
        let pin_model_collection = world.get::<PinModelCollection>(object.entity()).unwrap();

        view.insert(DeviceViewBundle::new(
            *position,
            *rotation,
            render_settings.binary_switch_extents,
        ))
        .with_children(|device| {
            device.spawn(BinarySwitchButtonBundle::new(
                render_settings,
                designer_assets.binary_switch_image.clone(),
            ));
            device.spawn(BinarySwitchBodyBundle::new(render_settings));
            device.spawn(BoardBinaryDisplayBundle::new(
                render_settings,
                common_assets,
                true,
            ));

            device
                .spawn(BinarySwitchPinCollectionBundle::new())
                .with_children(|pc| {
                    pc.spawn(BinarySwitchPinBundle::new(
                        render_settings,
                        pin_model_collection["Q"].uuid,
                    ));
                });
        });
    }
}

impl BuildView<DeviceViewKind> for BinaryDisplay {
    fn build(
        world: &World,
        object: Object<DeviceViewKind>,
        mut view: ViewCommands<DeviceViewKind>,
    ) {
        let common_assets = world.resource::<CommonAssets>();
        let render_settings = world.resource::<CircuitBoardRenderingSettings>();

        let position = world.get::<Position>(object.entity()).unwrap();
        let rotation = world.get::<Rotation>(object.entity()).unwrap();
        let pin_model_collection = world.get::<PinModelCollection>(object.entity()).unwrap();

        view.insert(DeviceViewBundle::new(
            *position,
            *rotation,
            render_settings.binary_display_extents,
        ))
        .with_children(|device| {
            device.spawn(BinaryDisplayBodyBundle::new(render_settings));
            device.spawn(BoardBinaryDisplayBundle::new(
                render_settings,
                common_assets,
                false,
            ));
            device
                .spawn(BinaryDisplayPinCollectionBundle::new())
                .with_children(|pc| {
                    pc.spawn(BinaryDisplayPinBundle::new(
                        render_settings,
                        pin_model_collection["Q"].uuid,
                    ));
                });
        });
    }
}

#[allow(clippy::type_complexity)]
pub fn update_board_binary_displays(
    q_board_binary_io: Query<
        (&PinModelCollection, &Viewable<DeviceViewKind>),
        (
            Or<(With<BinarySwitch>, With<BinaryDisplay>)>,
            Changed<PinModelCollection>,
        ),
    >,
    q_children: Query<&Children>,
    mut q_displays: Query<&mut Text2d, With<BoardBinaryDisplay>>,
) {
    for (pin_model_collection, viewable) in q_board_binary_io.iter() {
        let view_entity = viewable.view().entity();

        find_descendant!(
            q_children,
            view_entity,
            q_displays,
            |target: &mut Text2d| {
                target.0 = match pin_model_collection["Q"].signal_state.get_signal() {
                    Signal::High => "1".into(),
                    Signal::Low => "0".into(),
                    Signal::Conflict => "C".into(),
                };
            }
        );
    }
}

pub fn toggle_binary_switch(
    input: Res<ButtonInput<MouseButton>>,
    q_input_switches: Query<(Entity, &BoundingBox), With<BinarySwitchButton>>,
    q_cursor: Query<&Transform, With<Cursor>>,
    q_parents: Query<&Parent>,
    q_board_entities: Query<&View<DeviceViewKind>>,
    mut q_binary_switch_model: Query<&mut PinModelCollection, With<BinarySwitch>>,
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
                q_binary_switch_model,
                switch_entity
            ) else {
                return;
            };

            let current_signal = pin_collection["Q"].signal_state.get_signal().clone();
            pin_collection["Q"]
                .signal_state
                .set_signal(current_signal.negate());

            break;
        }
    }
}
