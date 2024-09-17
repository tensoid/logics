use bevy::prelude::*;

use crate::designer::{
    pin::{PinModel, PinModelCollection},
    position::Position,
};

use super::{device::Device, generic_chip::GenericChipBundle};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct And2;

impl Device for And2 {
    fn create_bundle(position: Position) -> impl Bundle {
        GenericChipBundle::new(
            position,
            PinModelCollection(vec![
                PinModel::new_input("B".into()),
                PinModel::new_input("A".into()),
                PinModel::new_output("Q".into()),
            ]),
            Self::device_id().into(),
        )
    }

    fn device_id() -> &'static str {
        "AND-2"
    }
}
