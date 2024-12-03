use bevy::prelude::*;
use moonshine_core::kind::Kind;
use moonshine_save::save::Save;
use moonshine_view::Viewable;

use crate::{
    designer::{
        bounding_box::BoundingBox,
        cursor::{Cursor, CursorState},
        model::Model,
        position::Position,
        selection::{Dragged, Selected},
    },
    events::events::SpawnDeviceEvent,
    get_cursor_mut,
};

pub trait Device: 'static + Send + Sync + Component {
    //TODO: add eval method (tick, simulate)
    fn create_bundle(position: Position) -> impl Bundle;
    fn device_id() -> &'static str;
}

pub trait RegisterDevice {
    fn register_device<T: Device>(&mut self) -> &mut Self;
}

impl RegisterDevice for App {
    fn register_device<T: Device>(&mut self) -> &mut Self {
        // register spawn func
        self.add_systems(
            Update,
            spawn_device::<T>.run_if(on_event::<SpawnDeviceEvent>()),
        );

        // store device_id in resource
        self.world_mut()
            .get_resource_or_insert_with::<DeviceIds>(DeviceIds::default)
            .devices
            .push(T::device_id().into());

        self
    }
}

fn spawn_device<T: Device>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnDeviceEvent>,
    q_selected_entities: Query<Entity, With<Selected>>,
    mut q_cursor: Query<&mut Cursor>,
) {
    for spawn_ev in spawn_events
        .read()
        .filter(|ev| ev.device_id == T::device_id())
    {
        let bundle = T::create_bundle(spawn_ev.position.clone());
        let entity = commands.spawn(bundle).id();

        if spawn_ev.init_drag {
            let mut cursor = get_cursor_mut!(q_cursor);

            for selected_entity in q_selected_entities.iter() {
                commands.entity(selected_entity).remove::<Selected>();
            }

            cursor.state = CursorState::Dragging;
            commands.entity(entity).insert(Selected);
            commands.entity(entity).insert(Dragged {
                cursor_offset: Position::ZERO,
            });
        }
    }
}

#[derive(Resource, Default)]
pub struct DeviceIds {
    pub devices: Vec<String>,
}

#[derive(Component)]
pub struct DeviceView;

#[derive(Bundle)]
pub struct DeviceViewBundle {
    device_view: DeviceView,
    bounding_box: BoundingBox,
    spatial_bundle: SpatialBundle,
}

impl DeviceViewBundle {
    pub fn new(position: Position, extents: Vec2) -> Self {
        Self {
            device_view: DeviceView,
            bounding_box: BoundingBox::rect_with_offset(
                extents / Vec2::new(2.0, 2.0),
                Vec2::ZERO,
                true,
            ),
            spatial_bundle: SpatialBundle {
                transform: Transform::from_xyz(position.0.x, position.0.y, 0.0),
                ..default()
            },
        }
    }
}

/// Marker component for device models
#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct DeviceModel;

#[derive(Bundle, Clone)]
pub struct DeviceModelBundle {
    device_model: DeviceModel,
    model: Model,
}

impl DeviceModelBundle {
    pub fn new(position: Position) -> Self {
        Self {
            device_model: DeviceModel,
            model: Model::from_position(position),
        }
    }
}

pub struct DeviceViewKind;

impl Kind for DeviceViewKind {
    type Filter = With<DeviceModel>;
}

pub fn update_device_positions(
    devices: Query<(&Viewable<DeviceViewKind>, &Position), Changed<Position>>,
    mut transform: Query<&mut Transform>,
) {
    for (viewable, position) in devices.iter() {
        let view = viewable.view();
        let mut transform = transform.get_mut(view.entity()).unwrap();
        *transform = Transform::from_translation(position.0.extend(transform.translation.z))
    }
}
