use bevy::prelude::*;

use super::{chip::ChipSpec, pin_state::PinState};

#[derive(Component)]
pub struct ChipInputPin {
    pub pin_state: PinState,
    pub input_received: bool,
}

#[derive(Component)]
pub struct ChipOutputPin(pub PinState);

#[derive(Component)]
pub struct BoardInputPin(pub PinState);

#[derive(Component)]
pub struct BoardOutputPin(pub PinState);

pub struct SpawnIOPinEvent {
    pub is_input: bool,
    pub position: Vec2,
}
