use bevy::prelude::*;

use self::events::{OpenChipSelectorEvent, SpawnChipEvent, SpawnIOPinEvent, ToggleDebugModeEvent};

pub mod events;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnChipEvent>()
            .add_event::<SpawnIOPinEvent>()
            .add_event::<OpenChipSelectorEvent>()
            .add_event::<ToggleDebugModeEvent>();
    }
}
