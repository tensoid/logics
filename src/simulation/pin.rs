use bevy::prelude::*;

use super::{chip::ChipSpec, pin_state::PinState};

//TODO: for color change, add marker component pin state changed and add a system that queries for it or event based

#[derive(Component)]
pub struct ChipInputPin(pub PinState);

#[derive(Component)]
pub struct ChipOutputPin(pub PinState);

impl ChipInputPin {
    pub fn num_input_pins_from_chip_spec(chip_spec: &ChipSpec) -> u16 {
        2
    }
}
