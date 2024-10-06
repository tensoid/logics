use bevy::prelude::*;

use crate::designer::{
    pin::{PinModel, PinModelCollection},
    position::Position,
};

use super::{device::Device, generic_chip::GenericChipBundle};

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct TFlipFlop;

impl Device for TFlipFlop {
    fn create_bundle(position: Position) -> impl Bundle {
        (
            TFlipFlop,
            GenericChipBundle::new(
                position,
                PinModelCollection(vec![
                    PinModel::new_input("C".into()),
                    PinModel::new_input("T".into()),
                    PinModel::new_output("Q".into()),
                ]),
                Self::device_id().into(),
            ),
        )
    }

    fn device_id() -> &'static str {
        "T-FF"
    }
}
