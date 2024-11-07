use bevy::prelude::*;
use events::{
    CopyEvent, LoadEvent, LoadRequestEvent, NewFileEvent, PasteEvent, SaveEvent, SaveRequestEvent,
    SelectAllEvent,
};

use self::events::{DeleteEvent, SpawnDeviceEvent, ToggleDebugModeEvent};

pub mod events;

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
