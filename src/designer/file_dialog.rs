use std::{
    env::{current_dir, current_exe},
    path::PathBuf,
};

use bevy::prelude::*;
use rfd::FileDialog;

use crate::events::events::{LoadEvent, SaveEvent};

#[derive(Resource, Default)]
pub struct ActiveSaveFile {
    pub path: Option<PathBuf>,
}

pub fn handle_save_request(
    mut active_save_file: ResMut<ActiveSaveFile>,
    mut save_ev_writer: EventWriter<SaveEvent>,
) {
    if let Some(active_save_file_path) = active_save_file.path.clone() {
        save_ev_writer.send(SaveEvent {
            path: active_save_file_path,
        });
        return;
    }

    let dialog_result = FileDialog::new()
        .add_filter("saves", &["ron"])
        .set_directory(get_saves_folder())
        .set_file_name("save.ron")
        .save_file();

    if let Some(path) = dialog_result {
        active_save_file.path = Some(path.clone());
        save_ev_writer.send(SaveEvent { path });
    }
}

pub fn handle_load_request(
    mut load_ev_writer: EventWriter<LoadEvent>,
    mut active_save_file: ResMut<ActiveSaveFile>,
) {
    let cwd = current_dir().unwrap();

    let dialog_result = FileDialog::new()
        .add_filter("saves", &["ron"])
        .set_directory(get_saves_folder())
        .pick_file();

    if let Some(path) = dialog_result {
        active_save_file.path = Some(path.clone());
        load_ev_writer.send(LoadEvent { path });
    }
}

fn get_saves_folder() -> PathBuf {
    let mut exe_path = current_exe().unwrap();
    exe_path.pop();
    exe_path.push("saves");
    exe_path
}
