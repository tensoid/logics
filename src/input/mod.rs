use bevy::prelude::*;

use self::keybindings::{handle_keybindings, Action, KeyBindings};

pub mod keybindings;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KeyBindings(vec![
            (vec![KeyCode::KeyD], Action::ToggleDebugMode),
            (vec![KeyCode::Delete], Action::DeleteSelected),
            (vec![KeyCode::ControlLeft, KeyCode::KeyS], Action::SaveBoard),
        ]))
        .add_systems(Update, handle_keybindings);
    }
}
