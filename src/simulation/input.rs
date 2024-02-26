use bevy::prelude::*;

use super::events::OpenChipSelectorEvent;

macro_rules! match_and_send_event {
    ($action:expr, $commands:expr, $($action_path:path => $event:path),* $(,)?) => {
        match $action {
            $(
                $action_path => $commands.add(|w: &mut World| {
                    w.send_event($event);
                }),
            )*
            _ => {} // Handle default case or remove this line if all actions are covered
        }
    };
}

pub enum Action {
    OpenChipSelector,
}

#[derive(Resource)]
pub struct KeyBindings(pub Vec<(Vec<KeyCode>, Action)>);

pub fn register_keybindings(app: &mut App) {
    app.insert_resource(KeyBindings(vec![(
        vec![KeyCode::Space],
        Action::OpenChipSelector,
    )]));
}

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
            Action::OpenChipSelector => OpenChipSelectorEvent,
        );
    }
}
