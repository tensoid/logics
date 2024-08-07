use bevy::prelude::*;

use crate::events::events::ToggleDebugModeEvent;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum DebugModeState {
    #[default]
    Off,
    On,
}

pub fn toggle_debug_mode(
    debug_mode_state: Res<State<DebugModeState>>,
    mut debug_mode_next_state: ResMut<NextState<DebugModeState>>,
    mut toggle_debug_mode_ev: EventReader<ToggleDebugModeEvent>,
) {
    println!("checl");
    for _ in toggle_debug_mode_ev.read() {
        println!("changd");
        let next_state = match debug_mode_state.get() {
            DebugModeState::Off => DebugModeState::On,
            DebugModeState::On => DebugModeState::Off,
        };
        debug_mode_next_state.set(next_state);
    }
}
