use std::path::Path;

use bevy::prelude::*;
use moonshine_save::FilePath;

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
