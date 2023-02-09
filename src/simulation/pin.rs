use bevy::prelude::*;

use super::{chip::ChipSpec, pin_state::PinState};

#[derive(Component)]
pub struct ChipInputPin(pub PinState);

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

impl ChipInputPin {
    pub fn num_input_pins_from_chip_spec(chip_spec: &ChipSpec) -> u16 {
        2
    }
}
