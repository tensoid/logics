use bevy::prelude::*;
use moonshine_save::save::Save;
use uuid::Uuid;

use super::position::Position;

#[derive(Component, Clone, Reflect)]
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
