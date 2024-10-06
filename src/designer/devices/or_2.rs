use bevy::prelude::*;
use uuid::Uuid;

use crate::designer::{
    pin::{PinModel, PinModelCollection},
    position::Position,
};

use super::{device::Device, generic_chip::GenericChipBundle};

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct Or2;

impl Device for Or2 {
    fn create_bundle(position: Position) -> impl Bundle {
        (
            Or2,
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
        "OR-2"
    }
}
