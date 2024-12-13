use std::collections::HashMap;

use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};
use moonshine_save::save::Save;
use uuid::Uuid;

use crate::events::{LoadEvent, NewFileEvent};

use super::position::Position;

#[derive(Clone, Reflect)]
#[reflect(Component)]
pub struct ModelId(pub Uuid);

impl ModelId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

// common stuff for all models
#[derive(Bundle, Clone)]
pub struct Model {
    //TODO: naming? is technically a bundle
    pub position: Position, //TODO: leave out? e.g. wires dont use position
    pub save: Save,
    pub id: ModelId,
}

impl Model {
    pub fn new() -> Self {
        Self {
            id: ModelId::new(),
            save: Save,
            position: Position::ZERO,
        }
    }

    pub fn from_position(position: Position) -> Self {
        Self {
            position,
            id: ModelId::new(),
            save: Save,
        }
    }
}

impl Component for ModelId {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _| {
            let uuid = world.get::<ModelId>(entity).unwrap().0;
            let mut model_registry = world.get_resource_mut::<ModelRegistry>().unwrap();
            model_registry.add_mapping(uuid, entity);
        });

        hooks.on_remove(|mut world, entity, _| {
            let mut model_registry = world.get_resource_mut::<ModelRegistry>().unwrap();
            model_registry.remove_by_entity(&entity);
        });
    }
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ModelRegistry {
    uuid_to_entity: HashMap<Uuid, Entity>,
    entity_to_uuid: HashMap<Entity, Uuid>,
}

//UNSURE: unwrap here and dont return option because registry should always be valid
#[allow(unused)]
impl ModelRegistry {
    pub fn get_model_entity(&self, uuid: &Uuid) -> Entity {
        *self
            .uuid_to_entity
            .get(uuid)
            .unwrap_or_else(|| panic!("Invalid Registry. Entity not found for uuid: {}", uuid))
    }

    pub fn try_get_model_entity(&self, uuid: &Uuid) -> Option<Entity> {
        self.uuid_to_entity.get(uuid).cloned()
    }

    pub fn get_model_uuid(&self, entity: &Entity) -> Uuid {
        *self
            .entity_to_uuid
            .get(entity)
            .unwrap_or_else(|| panic!("Invalid Registry. Uuid not found for entity: {}", entity))
    }

    pub fn try_get_model_uuid(&self, entity: &Entity) -> Option<Uuid> {
        self.entity_to_uuid.get(entity).cloned()
    }

    pub fn remove_by_uuid(&mut self, uuid: &Uuid) {
        let entity = self.get_model_entity(uuid);
        self.entity_to_uuid.remove(&entity);
        self.uuid_to_entity.remove(uuid);
    }

    pub fn remove_by_entity(&mut self, entity: &Entity) {
        let uuid = self.get_model_uuid(entity);
        self.entity_to_uuid.remove(entity);
        self.uuid_to_entity.remove(&uuid);
    }

    pub fn add_mapping(&mut self, uuid: Uuid, entity: Entity) {
        self.uuid_to_entity.insert(uuid, entity);
        self.entity_to_uuid.insert(entity, uuid);
    }

    pub fn clear(&mut self) {
        self.entity_to_uuid.clear();
        self.uuid_to_entity.clear();
    }
}
