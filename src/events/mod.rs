use bevy::prelude::*;
use moonshine_save::GetFilePath;

use std::path::{Path, PathBuf};

use crate::designer::position::Position;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeleteEvent>()
            .add_event::<SpawnDeviceEvent>()
            .add_event::<ToggleDebugModeEvent>()
            .add_event::<CopyEvent>()
            .add_event::<PasteEvent>()
            .add_event::<SelectAllEvent>()
            .add_event::<SaveEvent>()
            .add_event::<LoadEvent>()
            .add_event::<SaveRequestEvent>()
            .add_event::<LoadRequestEvent>()
            .add_event::<NewFileEvent>();
    }
}

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

impl GetFilePath for SaveEvent {
    fn path(&self) -> &Path {
        &self.path
    }
}

#[derive(Event, Clone)]
pub struct LoadEvent {
    pub path: PathBuf,
}

impl GetFilePath for LoadEvent {
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
