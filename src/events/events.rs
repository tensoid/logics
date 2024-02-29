use bevy::prelude::*;

#[derive(Event)]
pub struct SpawnChipEvent {
    pub chip_name: String,
    pub position: Vec2,
}

#[derive(Event)]
pub struct SpawnIOPinEvent {
    pub is_input: bool,
    pub position: Vec2,
}

#[derive(Event)]
pub struct OpenChipSelectorEvent;

#[derive(Event)]
pub struct ToggleDebugModeEvent;