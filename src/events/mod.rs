use bevy::prelude::*;
use events::{LoadEvent, SaveEvent};

use self::events::{DeleteSelectedEvent, SpawnBoardEntityEvent, ToggleDebugModeEvent};

pub mod events;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeleteSelectedEvent>()
            .add_event::<SpawnBoardEntityEvent>()
            .add_event::<ToggleDebugModeEvent>()
            .add_event::<SaveEvent>()
            .add_event::<LoadEvent>();
    }
}
