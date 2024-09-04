use bevy::prelude::*;

use crate::events::events::{
    CopyEvent, DeleteEvent, LoadEvent, PasteEvent, SaveEvent, ToggleDebugModeEvent,
};

macro_rules! match_and_send_event {
    ($action:expr, $commands:expr, $($action_path:path => $event:path),* $(,)?) => {
        match $action {
            $(
                $action_path => $commands.add(|w: &mut World| {
                    w.send_event($event);
                }),
            )*
        }
    };
}

pub enum Action {
    ToggleDebugMode,
    Delete,
    Copy,
    Paste,
    Save,
    Load,
}

#[derive(Resource)]
pub struct KeyBindings(pub Vec<(Vec<KeyCode>, Action)>);

pub fn handle_keybindings(
    mut commands: Commands,
    keybindings: Res<KeyBindings>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (keycodes, action) in keybindings.0.iter() {
        if !keycodes.iter().all(|key| input.pressed(*key)) {
            continue;
        }

        if !keycodes.iter().any(|key| input.just_pressed(*key)) {
            continue;
        }

        match_and_send_event!(
            action,
            commands,
            Action::ToggleDebugMode => ToggleDebugModeEvent,
            Action::Delete => DeleteEvent,
            Action::Copy => CopyEvent,
            Action::Paste => PasteEvent,
            Action::Save => SaveEvent,
            Action::Load => LoadEvent,
        );
    }
}
