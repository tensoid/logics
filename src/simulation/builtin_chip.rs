use bevy::prelude::*;

use super::chip::Evaluation;
use super::pin::Pin;
use super::pin_state::PinState;

#[derive(Component)]
pub struct BuiltinChip {
    pub input_pins: Vec<Pin>,
    pub output_pins: Vec<Pin>,
}

impl Evaluation for BuiltinChip {
    fn evaluate() -> bool {
        true
    }
}