use std::collections::HashMap;

use bevy::prelude::*;
use moonshine_save::save::Save;
use uuid::Uuid;

use crate::events::events::{CopyEvent, PasteEvent};

use super::{
    devices::device::DeviceModel,
    model::ModelId,
    pin::PinModelCollection,
    position::{self, Position},
    selection::Selected,
    signal::SignalState,
    wire::{
        wire_joint::{self, WireJointModel, WireJointModelBundle},
        WireModelBundle, WireNode, WireNodes,
    },
};

//TODO:
// maybe refactor by only having one copy and one paste function
// and a pipeline for update_position, update_model_uuids, init_drag etc... just passing the spawned entity ids
pub struct CopyPastePlugin;

impl Plugin for CopyPastePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DeviceClipboard>()
            .init_resource::<WireClipboard>()
            .init_resource::<WireJointClipboard>()
            .add_systems(
                Update,
                (copy_devices, copy_wire_joints, copy_wires).run_if(on_event::<CopyEvent>()),
            )
            .add_systems(
                Update,
                (paste_devices.pipe(paste_wire_joints).pipe(paste_wires))
                    .run_if(on_event::<PasteEvent>()),
            );
    }
}

/// Stores all components from all devices after copying using reflect.
#[derive(Resource, Default)]
pub struct DeviceClipboard {
    pub items: Vec<Vec<Box<dyn Reflect>>>,
}

/// Copies all devices and stores them in the [`DeviceClipboard`].
/// Currently copies all components on the device entity
/// that are registered in the type registry and can be reflected.
pub fn copy_devices(world: &mut World) {
    let entities: Vec<Entity> = world
        .query_filtered::<Entity, (With<Selected>, With<DeviceModel>)>()
        .iter(world)
        .collect();

    let mut copied_devices: Vec<Vec<Box<dyn Reflect>>> = Vec::new();

    {
        let type_registry = world.resource::<AppTypeRegistry>().read();
        for &entity in entities.iter() {
            let entity_ref = world.entity(entity);
            let device_components: Vec<Box<dyn Reflect>> = entity_ref
                .archetype()
                .components()
                .filter_map(|component_id| {
                    let type_id = world
                        .components()
                        .get_info(component_id)
                        .unwrap()
                        .type_id()
                        .unwrap();

                    type_registry.get(type_id).map(|type_registration| {
                        type_registration
                            .data::<ReflectComponent>()
                            .unwrap()
                            .reflect(entity_ref)
                            .unwrap()
                            .clone_value()
                    })
                })
                .collect();

            copied_devices.push(device_components);
        }
    }

    world.resource_mut::<DeviceClipboard>().items = copied_devices;
}

/// Spawns all devices stored in the [`DeviceClipboard`]
/// and selects them while deselecting all other currently selected devices.
pub fn paste_devices(world: &mut World) -> HashMap<Uuid, Uuid> {
    let mut q_selected = world.query_filtered::<Entity, With<Selected>>();
    let selected_entities: Vec<Entity> = q_selected.iter(world).collect();
    for entity in selected_entities {
        world.entity_mut(entity).remove::<Selected>();
    }

    let registry = world.resource::<AppTypeRegistry>().clone();
    let type_registry = registry.read();
    let mut spawned_entities: Vec<Entity> = Vec::new();

    world.resource_scope(|world, clipboard: Mut<DeviceClipboard>| {
        for clipboard_item in clipboard.items.iter() {
            let entity_mut = &mut world.spawn((Selected, Save)); // HACK: manually insert save because it does not implement the reflect trait and is not copied
            spawned_entities.push(entity_mut.id());
            for component in clipboard_item.iter() {
                let type_info = component.get_represented_type_info().unwrap();
                let registration = type_registry.get(type_info.type_id()).unwrap();
                let reflect_component = registration.data::<ReflectComponent>().unwrap();
                reflect_component.apply_or_insert(entity_mut, &**component, &type_registry);
            }
        }
    });

    // let mut q_cursor = world.query::<&Cursor>();
    // let cursor = q_cursor
    //     .get_single(world)
    //     .expect("No cursor entity in the scene.");

    //TODO: spawn relative to cursor
    for &entity in spawned_entities.iter() {
        let mut position = world
            .query::<&mut Position>()
            .get_mut(world, entity)
            .unwrap();

        position.0 += Vec2::new(50.0, -50.0);
    }

    // Randomize pin uuids and map old uuids to new uuids
    let mut pin_uuid_mapping: HashMap<Uuid, Uuid> = HashMap::new();

    for &entity in spawned_entities.iter() {
        let mut pin_model_collection = world
            .query::<&mut PinModelCollection>()
            .get_mut(world, entity)
            .unwrap();

        let device_pin_uuids: Vec<Uuid> = pin_model_collection
            .iter()
            .map(|pin_model| pin_model.uuid)
            .collect();

        pin_model_collection.randomize_pin_uuids();

        pin_model_collection.iter().zip(device_pin_uuids).for_each(
            |(new_pin_model, old_device_pin_uuid)| {
                pin_uuid_mapping.insert(old_device_pin_uuid, new_pin_model.uuid);
            },
        );
    }

    pin_uuid_mapping
}

/// Stores all wires after copying.
#[derive(Resource, Default)]
pub struct WireClipboard {
    pub items: Vec<WireNodes>,
}

/// Copies all wires and stores them in the [`WireClipboard`].
pub fn copy_wires(
    q_wires: Query<&WireNodes, With<Selected>>,
    mut wire_clipboard: ResMut<WireClipboard>,
) {
    wire_clipboard.items.clear();
    wire_clipboard.items = q_wires.iter().cloned().collect();
}

/// Spawns all wires stored in the [`WireClipboard`]
/// and updates the pin uuids to match the pasted devices.
pub fn paste_wires(
    In(uuid_mapping): In<HashMap<Uuid, Uuid>>,
    mut commands: Commands,
    wire_clipboard: Res<WireClipboard>,
) {
    for wire_nodes in wire_clipboard.items.iter() {
        let new_wire_nodes: Vec<WireNode> = wire_nodes
            .0
            .iter()
            .map(|wire_node| match wire_node {
                WireNode::Joint(uuid) => WireNode::Joint(*uuid_mapping.get(uuid).unwrap()),
                WireNode::Pin(uuid) => WireNode::Pin(*uuid_mapping.get(uuid).unwrap()),
            })
            .collect();

        commands.spawn((WireModelBundle::new(new_wire_nodes), Selected));
    }
}

/// Stores all wire joints after copying.
#[derive(Resource, Default)]
pub struct WireJointClipboard {
    pub items: Vec<(Position, Uuid)>,
}

/// Copies all wire joints and stores them in the [`WireJointClipboard`].
#[allow(clippy::type_complexity)]
pub fn copy_wire_joints(
    q_wire_joints: Query<(&Position, &ModelId), (With<Selected>, With<WireJointModel>)>,
    mut wire_joint_clipboard: ResMut<WireJointClipboard>,
) {
    wire_joint_clipboard.items.clear();
    wire_joint_clipboard.items = q_wire_joints
        .iter()
        .map(|(position, model_id)| (position.clone(), model_id.0))
        .collect();
}

/// Spawns all wire joints stored in the [`WireJointClipboard`]
pub fn paste_wire_joints(
    In(uuid_mapping): In<HashMap<Uuid, Uuid>>,
    wire_joint_clipboard: Res<WireJointClipboard>,
    mut commands: Commands,
) -> HashMap<Uuid, Uuid> {
    let mut uuid_mapping: HashMap<Uuid, Uuid> = uuid_mapping;

    for (joint_position, joint_uuid) in wire_joint_clipboard.items.iter() {
        let joint_position = Position(joint_position.0 + Vec2::new(50.0, -50.0));
        let new_joint = WireJointModelBundle::new(joint_position);
        uuid_mapping.insert(*joint_uuid, new_joint.model.id.0);
        commands.spawn((new_joint, Selected));
    }

    uuid_mapping
}
