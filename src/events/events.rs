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

#[derive(Event, Clone)]
pub struct ToggleDebugModeEvent;

#[derive(Event, Clone)]
pub struct DeleteEvent;

#[derive(Event, Clone)]
pub struct CopyEvent;

#[derive(Event, Clone)]
pub struct PasteEvent;

#[derive(Event, Clone)]
pub struct SaveEvent;

impl FilePath for SaveEvent {
    fn path(&self) -> &Path {
        Path::new("saves/save.ron")
    }
}

#[derive(Event, Clone)]
pub struct LoadEvent;

impl FilePath for LoadEvent {
    fn path(&self) -> &Path {
        Path::new("saves/save.ron")
    }
}

#[derive(Event, Clone)]
pub struct SelectAllEvent;
