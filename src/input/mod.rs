use bevy::prelude::*;

use crate::events::events::{
    CopyEvent, DeleteEvent, LoadEvent, LoadRequestEvent, PasteEvent, SaveEvent, SaveRequestEvent,
    SelectAllEvent, ToggleDebugModeEvent,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.register_keybinding(vec![KeyCode::KeyD], ToggleDebugModeEvent)
            .register_keybinding(vec![KeyCode::Delete], DeleteEvent)
            .register_keybinding(vec![KeyCode::ControlLeft, KeyCode::KeyC], CopyEvent)
            .register_keybinding(vec![KeyCode::ControlLeft, KeyCode::KeyV], PasteEvent)
            .register_keybinding(vec![KeyCode::ControlLeft, KeyCode::KeyS], SaveRequestEvent)
            .register_keybinding(vec![KeyCode::ControlLeft, KeyCode::KeyL], LoadRequestEvent)
            .register_keybinding(vec![KeyCode::ControlLeft, KeyCode::KeyA], SelectAllEvent);
    }
}

pub trait RegisterKeybinding {
    fn register_keybinding<E: Event + Clone>(
        &mut self,
        keybinding: Vec<KeyCode>,
        event: E,
    ) -> &mut Self;
}

impl RegisterKeybinding for App {
    fn register_keybinding<E: Event + Clone>(
        &mut self,
        keybinding: Vec<KeyCode>,
        event: E,
    ) -> &mut Self {
        self.add_systems(
            Update,
            move |input: Res<ButtonInput<KeyCode>>, event_writer: EventWriter<E>| {
                handle_keybinding(keybinding.clone(), event_writer, input, event.clone());
            },
        );

        self
    }
}

fn handle_keybinding<E: Event>(
    keybinding: Vec<KeyCode>,
    mut event_writer: EventWriter<E>,
    input: Res<ButtonInput<KeyCode>>,
    event: E,
) {
    if !keybinding.iter().all(|key| input.pressed(*key)) {
        return;
    }

    if !keybinding.iter().any(|key| input.just_pressed(*key)) {
        return;
    }

    event_writer.send(event);
}
