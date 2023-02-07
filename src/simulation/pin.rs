use bevy::prelude::*;

use super::{chip::ChipSpec, pin_state::PinState};

#[derive(Component)]
pub struct ChipInputPin(pub PinState);

#[derive(Component)]
pub struct ChipOutputPin(pub PinState);

impl ChipInputPin {
    pub fn num_input_pins_from_chip_spec(chip_spec: &ChipSpec) -> u16 {
        2
    }
}
