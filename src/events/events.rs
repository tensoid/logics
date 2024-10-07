use std::path::Path;

use bevy::prelude::*;
use moonshine_save::FilePath;

use crate::designer::position::Position;

#[derive(Event, Clone)]
pub struct SpawnDeviceEvent {
    pub device_id: String,
    pub position: Position,
    pub init_drag: bool,
}

#[derive(Event)]
pub struct ToggleDebugModeEvent;

#[derive(Event)]
pub struct DeleteEvent;

#[derive(Event)]
pub struct CopyEvent;

#[derive(Event)]
pub struct PasteEvent;

#[derive(Event)]
pub struct SaveEvent;

impl FilePath for SaveEvent {
    fn path(&self) -> &Path {
        Path::new("saves/save.ron")
    }
}

#[derive(Event)]
pub struct LoadEvent;

impl FilePath for LoadEvent {
    fn path(&self) -> &Path {
        Path::new("saves/save.ron")
    }
}

#[derive(Event)]
pub struct SelectAllEvent;
