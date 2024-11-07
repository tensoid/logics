use std::path::{Path, PathBuf};

use bevy::prelude::*;
use moonshine_save::FilePath;

use crate::designer::position::Position;

#[derive(Event, Clone)]
pub struct SpawnDeviceEvent {
    pub device_id: String,
    pub position: Position,
    pub init_drag: bool,
}

//TODO: prefix designer events

#[derive(Event, Clone)]
pub struct ToggleDebugModeEvent;

#[derive(Event, Clone)]
pub struct DeleteEvent;

#[derive(Event, Clone)]
pub struct CopyEvent;

#[derive(Event, Clone)]
pub struct PasteEvent;

#[derive(Event, Clone)]
pub struct SelectAllEvent;

#[derive(Event, Clone)]
pub struct SaveEvent {
    pub path: PathBuf,
}

impl FilePath for SaveEvent {
    fn path(&self) -> &Path {
        &self.path
    }
}

#[derive(Event, Clone)]
pub struct LoadEvent {
    pub path: PathBuf,
}

impl FilePath for LoadEvent {
    fn path(&self) -> &Path {
        &self.path
    }
}

#[derive(Event, Clone)]
pub struct SaveRequestEvent;

#[derive(Event, Clone)]
pub struct LoadRequestEvent;

#[derive(Event, Clone)]
pub struct NewFileEvent;
