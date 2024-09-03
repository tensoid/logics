use bevy::prelude::*;
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::ShapeBundle,
    prelude::GeometryBuilder,
    shapes,
};
use moonshine_core::object::{Object, ObjectInstance};
use moonshine_view::{BuildView, ViewCommands};
use uuid::Uuid;

use crate::events::events::SpawnBoardEntityEvent;

use super::{
    board_entity::{BoardEntityModelBundle, BoardEntityViewBundle, BoardEntityViewKind, Position},
    pin::{PinCollectionBundle, PinModel, PinModelCollection, PinViewBundle},
    render_settings::CircuitBoardRenderingSettings,
};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Clock {
    timer: Timer,
}

impl Clock {
    pub fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
        }
    }
}

#[derive(Bundle)]
pub struct ClockBundle {
    clock: Clock,
    model_bundle: BoardEntityModelBundle,
    pin_model_collection: PinModelCollection,
}

impl ClockBundle {
    fn new(position: Position, timer_seconds: f32) -> Self {
        Self {
            clock: Clock::new(timer_seconds),
            model_bundle: BoardEntityModelBundle::new(position),
            pin_model_collection: PinModelCollection(vec![PinModel::new_output(
                "Q".into(),
                Uuid::new_v4(),
            )]),
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
        let points = vec![
            Vec2::new(-1.0, -1.0),
            Vec2::new(-1.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, -1.0),
        ]
        .into_iter()
        .map(|x| x * (render_settings.board_binary_output_extents / 2.0)) // TODO: into settings
        .collect();

        Self {
            clock_body: ClockBody,
            fill: Fill::color(render_settings.board_binary_io_color), // TODO: into settings
            stroke: Stroke::new(
                render_settings.board_entity_stroke_color,
                render_settings.board_entity_stroke_width,
            ),
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::RoundedPolygon {
                    points,
                    radius: render_settings.board_entity_edge_radius,
                    closed: false,
                }),
                spatial: SpatialBundle::default(),
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
    text_bundle: Text2dBundle,
}

impl ClockLabelBundle {
    fn new(render_settings: &CircuitBoardRenderingSettings, asset_server: &AssetServer) -> Self {
        let font: Handle<Font> = asset_server.load("fonts/VCR_OSD_MONO.ttf");

        let text_style = TextStyle {
            font_size: render_settings.board_binary_io_display_font_size, // TODO: settings
            color: Color::BLACK,
            font,
        };

        Self {
            clock_label: ClockLabel,
            text_bundle: Text2dBundle {
                text: Text::from_section("C", text_style).with_justify(JustifyText::Center),
                transform: Transform::from_xyz(0.0, 0.0, 0.01),
                ..default()
            },
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
                render_settings.board_binary_io_pin_radius, // TODO: into settings
                Vec3::new(
                    render_settings.board_binary_output_extents.x / 2.0, // TODO: into settings
                    0.0,
                    0.02,
                ),
            ),
        }
    }
}

#[derive(Component)]
struct ClockPinCollection;

#[derive(Bundle)]
struct ClockPinCollectionBundle {
    clock_pin_collection: ClockPinCollection,
    pin_collection_bundle: PinCollectionBundle,
}

impl ClockPinCollectionBundle {
    fn new() -> Self {
        Self {
            clock_pin_collection: ClockPinCollection,
            pin_collection_bundle: PinCollectionBundle::new(),
        }
    }
}

pub fn spawn_clock(
    mut commands: Commands,
    mut spawn_ev: EventReader<SpawnBoardEntityEvent>,
) -> Option<(Entity, SpawnBoardEntityEvent)> {
    for ev in spawn_ev.read() {
        if ev.name != "CLOCK" {
            continue;
        }

        let entity = commands
            .spawn(ClockBundle::new(ev.position.clone(), 0.2))
            .id();

        return Some((entity, ev.clone()));
    }

    None
}

impl BuildView<BoardEntityViewKind> for Clock {
    fn build(
        world: &World,
        object: Object<BoardEntityViewKind>,
        view: &mut ViewCommands<BoardEntityViewKind>,
    ) {
        let render_settings = world.resource::<CircuitBoardRenderingSettings>();
        let asset_server = world.resource::<AssetServer>();

        let position = world.get::<Position>(object.entity()).unwrap();
        let pin_model_collection = world.get::<PinModelCollection>(object.entity()).unwrap();

        view.insert(BoardEntityViewBundle::new(
            position.clone(),
            render_settings.board_binary_output_extents, // TODO: into settings
        ))
        .with_children(|board_entity| {
            board_entity.spawn(ClockBodyBundle::new(render_settings));
            board_entity.spawn(ClockLabelBundle::new(render_settings, asset_server));

            board_entity
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
            pin_model_collection["Q"].next_signal_state = !pin_model_collection["Q"].signal_state;
        }
    }
}
