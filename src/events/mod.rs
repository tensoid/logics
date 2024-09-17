use bevy::prelude::*;
use events::{CopyEvent, LoadEvent, PasteEvent, SaveEvent};

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
            .add_event::<SaveEvent>()
            .add_event::<LoadEvent>();
    }
}
