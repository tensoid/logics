use bevy::prelude::*;

use self::events::{
    DeleteSelectedEvent, OpenChipSelectorEvent, SpawnBoardEntityEvent, ToggleDebugModeEvent,
};

pub mod events;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OpenChipSelectorEvent>()
            .add_event::<DeleteSelectedEvent>()
            .add_event::<SpawnBoardEntityEvent>()
            .add_event::<ToggleDebugModeEvent>();
    }
}
