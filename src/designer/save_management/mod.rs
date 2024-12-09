use crate::events::events::{
    LoadEvent, LoadRequestEvent, NewFileEvent, SaveEvent, SaveRequestEvent,
};
use bevy::prelude::*;
use bevy_file_dialog::{DialogFilePicked, FileDialogExt, FileDialogPlugin};
use moonshine_save::{load::load_from_file_on_event, save::save_default};
use std::{env::current_exe, path::PathBuf};

use super::devices::device::DeviceModel;

struct LoadFilePick;
struct SaveFilePick;

//UNSURE: might be better outside of designer
pub struct SaveManagementPlugin;

impl Plugin for SaveManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_save_request.run_if(on_event::<SaveRequestEvent>()),
        )
        .add_systems(
            Update,
            handle_load_request.run_if(on_event::<LoadRequestEvent>()),
        )
        .add_systems(Update, new_file.run_if(on_event::<NewFileEvent>()))
        .add_systems(
            PreUpdate,
            (
                // Needs additional on_event condition because of the use of has_event in the moonshine_save crate.
                // has_event doesnt consume the event and because of that it executes the pipeline multiple times per event which causes a crash.
                // This might be fixed in the latest version of moonshine_save which is not yet published on crates io.
                save_default()
                    .into_file_on_event::<SaveEvent>()
                    .run_if(on_event::<SaveEvent>()),
                load_from_file_on_event::<LoadEvent>().run_if(on_event::<LoadEvent>()),
            ),
        )
        .add_plugins(
            FileDialogPlugin::new()
                .with_pick_file::<SaveFilePick>()
                .with_pick_file::<LoadFilePick>(),
        )
        .add_systems(Update, (save_file_picked, load_file_picked))
        .init_resource::<ActiveSaveFile>();
    }
}

#[derive(Resource, Default)]
pub struct ActiveSaveFile {
    pub path: Option<PathBuf>,
}

pub fn new_file(
    q_entities: Query<Entity, With<DeviceModel>>,
    mut active_save_file: ResMut<ActiveSaveFile>,
    mut commands: Commands,
) {
    //TODO: maybe moonshine function for this
    for entity in q_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    active_save_file.path = None;
}

/// Saves the scene into the currently active save file if exists,
/// otherwise opens a file dialog to save into a new file.
pub fn handle_save_request(
    active_save_file: Res<ActiveSaveFile>,
    mut save_ev_writer: EventWriter<SaveEvent>,
    mut commands: Commands,
) {
    if let Some(active_save_file_path) = active_save_file.path.clone() {
        save_ev_writer.send(SaveEvent {
            path: active_save_file_path,
        });
        return;
    }

    commands
        .dialog()
        .add_filter("saves", &["ron"])
        .set_directory(get_saves_folder())
        .set_file_name("save.ron")
        .pick_file_path::<SaveFilePick>();
}

fn save_file_picked(
    mut ev_saved: EventReader<DialogFilePicked<SaveFilePick>>,
    mut save_ev_writer: EventWriter<SaveEvent>,
    mut active_save_file: ResMut<ActiveSaveFile>,
) {
    for ev in ev_saved.read() {
        let path = ev.path.clone();
        active_save_file.path = Some(path.clone());
        save_ev_writer.send(SaveEvent { path });
    }
}

/// Opens a file dialog to select a save file to load.
pub fn handle_load_request(mut commands: Commands) {
    commands
        .dialog()
        .add_filter("saves", &["ron"])
        .set_directory(get_saves_folder())
        .pick_file_path::<LoadFilePick>();
}

fn load_file_picked(
    mut ev_loaded: EventReader<DialogFilePicked<LoadFilePick>>,
    mut active_save_file: ResMut<ActiveSaveFile>,
    mut load_ev_writer: EventWriter<LoadEvent>,
) {
    for ev in ev_loaded.read() {
        let path = ev.path.clone();
        active_save_file.path = Some(path.clone());
        load_ev_writer.send(LoadEvent { path });
    }
}

/// Gets the "saves" folder that is relative to the executable.
fn get_saves_folder() -> PathBuf {
    let mut exe_path = current_exe().unwrap();
    exe_path.pop();
    exe_path.push("saves");
    exe_path
}
