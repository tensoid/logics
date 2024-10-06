use std::collections::HashSet;

use bevy::prelude::*;
use uuid::Uuid;

use super::{
    devices::device::{DeviceModel, DeviceModelBundle},
    pin::{self, PinModelCollection},
    position::Position,
    selection::Selected,
    signal_state::SignalState,
    wire::{Wire, WireBundle},
};

//TODO: randomize pin uuids
//TODO: figure out how to find new pin uuids
//TODO: comments

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
pub fn paste_devices(world: &mut World) {
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
            let entity_mut = &mut world.spawn(Selected);
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
}

/// Stores all wires after copying.
#[derive(Resource, Default)]
pub struct WireClipboard {
    pub items: Vec<(Wire, SignalState)>,
}

pub fn copy_wires(
    q_wires: Query<(&Wire, &SignalState)>,
    q_selected_devices: Query<&PinModelCollection, (With<Selected>, With<DeviceModel>)>,
    mut wire_clipboard: ResMut<WireClipboard>,
) {
    wire_clipboard.items.clear();

    for (wire, signal_state) in q_wires.iter() {
        // find wires connected to copied devices
        let (Some(wire_src_pin_uuid), Some(wire_dest_pin_uuid)) =
            (wire.src_pin_uuid, wire.dest_pin_uuid)
        else {
            continue;
        };

        let pin_uuids: HashSet<Uuid> = HashSet::from_iter(
            q_selected_devices
                .iter()
                .flat_map(|pin_model_collection| pin_model_collection.0.clone())
                .map(|pin_model| pin_model.uuid),
        );

        if !pin_uuids.contains(&wire_src_pin_uuid) || !pin_uuids.contains(&wire_dest_pin_uuid) {
            continue;
        }

        // copy connected wires
        wire_clipboard.items.push((wire.clone(), *signal_state));
    }
    info!("{}", wire_clipboard.items.len());
}

pub fn paste_wires(mut commands: Commands, wire_clipboard: Res<WireClipboard>) {
    for wire in wire_clipboard.items.iter() {
        commands.spawn(WireBundle::new_with_signal(wire.0.clone(), wire.1));
    }
}
