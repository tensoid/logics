use std::collections::HashMap;

use bevy::prelude::*;
use moonshine_save::save::Save;
use uuid::Uuid;

use crate::events::events::{CopyEvent, PasteEvent};

use super::{
    devices::device::DeviceModel, pin::PinModelCollection, position::Position, selection::Selected,
    signal::SignalState, wire::WireNodes,
};

pub struct CopyPastePlugin;

impl Plugin for CopyPastePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DeviceClipboard>()
            .init_resource::<WireClipboard>()
            .add_systems(
                Update,
                (copy_devices, copy_wires).run_if(on_event::<CopyEvent>()),
            )
            .add_systems(
                Update,
                (paste_devices.pipe(paste_wires)).run_if(on_event::<PasteEvent>()),
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
        .query_filtered::<Entity, With<Selected>>()
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
            let entity_mut = &mut world.spawn((Selected, Save)); // FIXME: manually insert save because it does not implement the reflect trait and is not copied
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
    pub items: Vec<(WireNodes, SignalState)>,
}

/// Copies all wires and stores them in the [`WireClipboard`].
pub fn copy_wires(// q_wires: Query<(&WireModel, &SignalState)>,
    // q_selected_devices: Query<&PinModelCollection, (With<Selected>, With<DeviceModel>)>,
    // mut wire_clipboard: ResMut<WireClipboard>,
) {
    // wire_clipboard.items.clear();

    // for (wire, signal_state) in q_wires.iter() {
    //     // find wires connected to copied devices
    //     let (Some(wire_src_pin_uuid), Some(wire_dest_pin_uuid)) =
    //         (wire.src_pin_uuid, wire.dest_pin_uuid)
    //     else {
    //         continue;
    //     };

    //     let pin_uuids: HashSet<Uuid> = HashSet::from_iter(
    //         q_selected_devices
    //             .iter()
    //             .flat_map(|pin_model_collection| pin_model_collection.0.clone())
    //             .map(|pin_model| pin_model.uuid),
    //     );

    //     if !pin_uuids.contains(&wire_src_pin_uuid) || !pin_uuids.contains(&wire_dest_pin_uuid) {
    //         continue;
    //     }

    //     // copy connected wires
    //     wire_clipboard.items.push((wire.clone(), *signal_state));
    // }
}

/// Spawns all wires stored in the [`WireClipboard`]
/// and updates the pin uuids to match the pasted devices.
pub fn paste_wires(
    In(pin_uuid_mapping): In<HashMap<Uuid, Uuid>>,
    mut commands: Commands,
    wire_clipboard: Res<WireClipboard>,
) {
    // for (wire, signal_state) in wire_clipboard.items.iter() {
    //     let mut new_wire = wire.clone();
    //     new_wire.src_pin_uuid = Some(
    //         *pin_uuid_mapping
    //             .get(&new_wire.src_pin_uuid.unwrap())
    //             .unwrap(),
    //     );
    //     new_wire.dest_pin_uuid = Some(
    //         *pin_uuid_mapping
    //             .get(&new_wire.dest_pin_uuid.unwrap())
    //             .unwrap(),
    //     );

    //     commands.spawn(WireBundle::new_with_signal(new_wire, *signal_state));
    // }
}
