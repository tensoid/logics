use bevy::prelude::*;
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::ShapeBundle,
    prelude::GeometryBuilder,
    shapes::{self, BorderRadii},
};
use moonshine_core::object::{Object, ObjectInstance};
use moonshine_view::{BuildView, ViewCommands};
use uuid::Uuid;

use crate::{
    assets::common_assets::CommonAssets,
    designer::{
        pin::{PinModel, PinModelCollection, PinViewBundle, PinViewCollectionBundle},
        position::Position,
        render_settings::CircuitBoardRenderingSettings,
    },
};

use super::device::{Device, DeviceModelBundle, DeviceViewBundle, DeviceViewKind};

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct Clock {
    timer: Timer,
}

impl Device for Clock {
    fn create_bundle(position: Position) -> impl Bundle {
        ClockBundle::new(position, 0.2)
    }

    fn device_id() -> &'static str {
        "CLOCK"
    }
}

impl Clock {
    pub fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
        }
    }
}

#[derive(Bundle, Clone)]
pub struct ClockBundle {
    clock: Clock,
    model_bundle: DeviceModelBundle,
    pin_model_collection: PinModelCollection,
}

impl ClockBundle {
    fn new(position: Position, timer_seconds: f32) -> Self {
        Self {
            clock: Clock::new(timer_seconds),
            model_bundle: DeviceModelBundle::new(position),
            pin_model_collection: PinModelCollection(vec![PinModel::new_output("Q".into())]),
        }
    }
}

#[derive(Component)]
pub struct ClockBody;

#[derive(Bundle)]
pub struct ClockBodyBundle {
    clock_body: ClockBody,
    fill: Fill,
    stroke: Stroke,
    shape_bundle: ShapeBundle,
}

impl ClockBodyBundle {
    fn new(render_settings: &CircuitBoardRenderingSettings) -> Self {
        Self {
            clock_body: ClockBody,
            fill: Fill::color(render_settings.clock_color),
            stroke: Stroke::new(
                render_settings.device_stroke_color,
                render_settings.device_stroke_width,
            ),
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: render_settings.clock_extents,
                    radii: Some(BorderRadii::single(render_settings.device_border_radius)),
                    ..default()
                }),
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct ClockLabel;

#[derive(Bundle)]
pub struct ClockLabelBundle {
    clock_label: ClockLabel,
    text_2d: Text2d,
    text_font: TextFont,
    text_color: TextColor,
    text_layout: TextLayout,
    transform: Transform,
}

impl ClockLabelBundle {
    fn new(render_settings: &CircuitBoardRenderingSettings, common_assets: &CommonAssets) -> Self {
        Self {
            clock_label: ClockLabel,
            text_2d: Text2d::new("C"),
            text_color: TextColor(Color::BLACK),
            text_font: TextFont {
                font: common_assets.font.clone(),
                font_size: render_settings.clock_label_font_size,
                ..default()
            },
            text_layout: TextLayout::new_with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 0.0, 0.01),
        }
    }
}

#[derive(Component)]
pub struct ClockPin;

#[derive(Bundle)]
pub struct ClockPinBundle {
    clock_pin: ClockPin,
    pin_view_bundle: PinViewBundle,
}

impl ClockPinBundle {
    fn new(render_settings: &CircuitBoardRenderingSettings, uuid: Uuid) -> Self {
        Self {
            clock_pin: ClockPin,
            pin_view_bundle: PinViewBundle::new(
                render_settings,
                uuid,
                render_settings.clock_pin_radius,
                Vec3::new(render_settings.clock_extents.x / 2.0, 0.0, 0.02),
            ),
        }
    }
}

#[derive(Component)]
struct ClockPinCollection;

#[derive(Bundle)]
struct ClockPinCollectionBundle {
    clock_pin_collection: ClockPinCollection,
    pin_collection_bundle: PinViewCollectionBundle,
}

impl ClockPinCollectionBundle {
    fn new() -> Self {
        Self {
            clock_pin_collection: ClockPinCollection,
            pin_collection_bundle: PinViewCollectionBundle::new(),
        }
    }
}

impl BuildView<DeviceViewKind> for Clock {
    fn build(
        world: &World,
        object: Object<DeviceViewKind>,
        mut view: ViewCommands<DeviceViewKind>,
    ) {
        let render_settings = world.resource::<CircuitBoardRenderingSettings>();
        let common_assets = world.resource::<CommonAssets>();

        let position = world.get::<Position>(object.entity()).unwrap();
        let pin_model_collection = world.get::<PinModelCollection>(object.entity()).unwrap();

        view.insert(DeviceViewBundle::new(
            position.clone(),
            render_settings.clock_extents,
        ))
        .with_children(|device| {
            device.spawn(ClockBodyBundle::new(render_settings));
            device.spawn(ClockLabelBundle::new(render_settings, common_assets));

            device
                .spawn(ClockPinCollectionBundle::new())
                .with_children(|pc| {
                    pc.spawn(ClockPinBundle::new(
                        render_settings,
                        pin_model_collection["Q"].uuid,
                    ));
                });
        });
    }
}

pub fn tick_clocks(mut q_clocks: Query<(&mut Clock, &mut PinModelCollection)>, time: Res<Time>) {
    for (mut clock, mut pin_model_collection) in q_clocks.iter_mut() {
        clock.timer.tick(time.delta());

        if clock.timer.finished() {
            let current_signal = pin_model_collection["Q"].signal_state.get_signal().clone();
            pin_model_collection["Q"]
                .signal_state
                .set_signal(current_signal.negate());
        }
    }
}
