use bevy::prelude::*;

pub struct SpawnChipEvent {
    pub chip_name: String,
    pub position: Vec2,
}

pub struct SpawnIOPinEvent {
    pub is_input: bool,
    pub position: Vec2,
}

pub struct OpenChipSelectorEvent;

pub fn register_events(app: &mut App) {
    app.add_event::<SpawnChipEvent>()
        .add_event::<SpawnIOPinEvent>()
        .add_event::<OpenChipSelectorEvent>();
}
