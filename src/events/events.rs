use bevy::prelude::*;

#[derive(Event, Clone)]
pub struct SpawnBoardEntityEvent {
    pub name: String,
    pub position: Vec2,
    pub init_drag: bool,
}

#[derive(Event)]
pub struct ToggleDebugModeEvent;

#[derive(Event)]
pub struct DeleteSelectedEvent;
