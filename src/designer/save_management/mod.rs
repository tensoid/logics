use std::{env::current_exe, path::PathBuf};

use bevy::{prelude::*, tasks::AsyncComputeTaskPool, window::PrimaryWindow};
use moonshine_save::{
    file_from_event,
    load::{load, unload},
    save::{save_default, Save},
};
use rfd::AsyncFileDialog;

use crossbeam_channel::{bounded, Receiver, Sender};

use crate::events::{LoadEvent, LoadRequestEvent, NewFileEvent, SaveEvent, SaveRequestEvent};

//UNSURE: might be better outside of designer
pub struct SaveManagementPlugin;

impl Plugin for SaveManagementPlugin {
    fn build(&self, app: &mut App) {
        // pick save file
        let (stx, srx) = bounded::<SaveFilePick>(1);
        app.insert_resource(AsyncSender(stx));
        app.insert_resource(AsyncReceiver(srx));
        app.add_systems(First, handle_save_file_picked_result);
        app.add_systems(
            Update,
            handle_save_request.run_if(on_event::<SaveRequestEvent>),
        );

        // pick load file
        let (ltx, lrx) = bounded::<LoadFilePick>(1);
        app.insert_resource(AsyncSender(ltx));
        app.insert_resource(AsyncReceiver(lrx));
        app.add_systems(First, handle_load_file_picked_result);
        app.add_systems(
            Update,
            handle_load_request.run_if(on_event::<LoadRequestEvent>),
        );

        app.add_systems(
            Update,
            update_window_title.run_if(resource_changed::<ActiveSaveFile>),
        );

        app.init_resource::<ActiveSaveFile>();

        app.add_systems(
            First,
            (
                // Needs additional on_event condition because of the use of has_event in the moonshine_save crate.
                // has_event doesnt consume the event and because of that it executes the pipeline multiple times per event which causes a crash.
                // This might be fixed in the latest version of moonshine_save which is not yet published on crates io.
                save_default()
                    .into(file_from_event::<SaveEvent>())
                    .run_if(on_event::<SaveEvent>),
                load(file_from_event::<LoadEvent>()).run_if(on_event::<LoadEvent>),
            ),
        );
        app.add_systems(Update, new_file.run_if(on_event::<NewFileEvent>));
    }
}

#[derive(Resource, Default)]
pub struct ActiveSaveFile {
    pub path: Option<PathBuf>,
}

#[derive(Deref)]
struct SaveFilePick(pub PathBuf);
#[derive(Deref)]
struct LoadFilePick(pub PathBuf);

#[derive(Resource, Deref)]
struct AsyncReceiver<T>(Receiver<T>);

#[derive(Resource, Deref)]
struct AsyncSender<T>(Sender<T>);

pub fn new_file(
    q_entities: Query<Entity, With<Save>>,
    mut active_save_file: ResMut<ActiveSaveFile>,
    mut commands: Commands,
) {
    for entity in q_entities.iter() {
        commands.entity(entity).despawn_recursive(); //TODO: try insert unload instead
    }

    active_save_file.path = None;
}

fn handle_save_file_picked_result(
    receiver: Res<AsyncReceiver<SaveFilePick>>,
    mut active_save_file: ResMut<ActiveSaveFile>,
    mut save_ev_writer: EventWriter<SaveEvent>,
) {
    for result in receiver.try_iter() {
        active_save_file.path = Some(result.clone());
        save_ev_writer.send(SaveEvent {
            path: result.clone(),
        });
    }
}

fn handle_load_file_picked_result(
    receiver: Res<AsyncReceiver<LoadFilePick>>,
    mut active_save_file: ResMut<ActiveSaveFile>,
    mut load_ev_writer: EventWriter<LoadEvent>,
) {
    for result in receiver.try_iter() {
        active_save_file.path = Some(result.clone());
        load_ev_writer.send(LoadEvent {
            path: result.clone(),
        });
    }
}

fn handle_save_request(
    sender: Res<AsyncSender<SaveFilePick>>,
    active_save_file: Res<ActiveSaveFile>,
    mut save_ev_writer: EventWriter<SaveEvent>,
) {
    if let Some(active_save_file_path) = active_save_file.path.clone() {
        save_ev_writer.send(SaveEvent {
            path: active_save_file_path,
        });
        return;
    }

    let sender = sender.clone();

    AsyncComputeTaskPool::get()
        .spawn(async move {
            let result = AsyncFileDialog::new()
                .add_filter("saves", &["ron"])
                .set_directory(get_saves_folder())
                .set_file_name("save.ron")
                .save_file()
                .await;

            if let Some(file_handle) = result {
                sender
                    .send(SaveFilePick(file_handle.path().to_path_buf()))
                    .unwrap();
            }
        })
        .detach();
}

fn handle_load_request(sender: Res<AsyncSender<LoadFilePick>>) {
    let sender = sender.clone();

    AsyncComputeTaskPool::get()
        .spawn(async move {
            let result = AsyncFileDialog::new()
                .add_filter("saves", &["ron"])
                .set_directory(get_saves_folder())
                .pick_file()
                .await;

            if let Some(file_handle) = result {
                sender
                    .send(LoadFilePick(file_handle.path().to_path_buf()))
                    .unwrap();
            }
        })
        .detach();
}

/// Gets the "saves" folder that is relative to the executable.
fn get_saves_folder() -> PathBuf {
    let mut exe_path = current_exe().unwrap();
    exe_path.pop();
    exe_path.push("saves");
    exe_path
}

// Update window title to current file
fn update_window_title(
    active_save_file: Res<ActiveSaveFile>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = q_window.get_single_mut().unwrap();
    window.title = match &active_save_file.path {
        Some(path) => path.file_name().unwrap().to_str().unwrap().to_string(),
        None => "New File".into(),
    }
}
