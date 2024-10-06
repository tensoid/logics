use bevy::prelude::*;

use crate::designer::{
    pin::{PinModel, PinModelCollection},
    position::Position,
};

use super::{device::Device, generic_chip::GenericChipBundle};

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct Nand2;

impl Device for Nand2 {
    fn create_bundle(position: Position) -> impl Bundle {
        (
            Nand2,
            GenericChipBundle::new(
                position,
                PinModelCollection(vec![
                    PinModel::new_input("B".into()),
                    PinModel::new_input("A".into()),
                    PinModel::new_output("Q".into()),
                ]),
                Self::device_id().into(),
            ),
        )
    }

    fn device_id() -> &'static str {
        "NAND-2"
    }
}
