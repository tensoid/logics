pub mod and_2;
pub mod binary_io;
pub mod clock;
pub mod d_flipflop;
pub mod device;
pub mod generic_chip;
pub mod jk_flipflop;
pub mod nand_2;
pub mod not;
pub mod or_2;
pub mod t_flipflop;
pub mod xor_2;

use and_2::And2;
use bevy::prelude::*;
use binary_io::{toggle_binary_switch, update_board_binary_displays, BinaryDisplay, BinarySwitch};
use clock::{tick_clocks, Clock};
use d_flipflop::DFlipFlop;
use device::{update_device_positions, DeviceModel, DeviceViewKind, RegisterDevice};
use generic_chip::GenericChip;
use jk_flipflop::JKFlipFlop;
use moonshine_view::RegisterView;
use nand_2::Nand2;
use not::Not;
use or_2::Or2;
use t_flipflop::TFlipFlop;
use xor_2::Xor2;

use crate::simulation::simulation::propagate_signals;

use super::{
    pin::PinModelCollection, position::Position, signal::SignalState, wire::WireNodes,
};

pub struct DevicePlugin;

impl Plugin for DevicePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DeviceModel>()
            .register_type::<Position>()
            .register_type::<BinarySwitch>()
            .register_type::<BinaryDisplay>()
            .register_type::<GenericChip>()
            .register_type::<PinModelCollection>()
            .register_type::<Clock>();

        app.register_view::<DeviceViewKind, BinarySwitch>()
            .register_view::<DeviceViewKind, BinaryDisplay>()
            .register_view::<DeviceViewKind, GenericChip>()
            .register_view::<DeviceViewKind, Clock>();

        app.register_device::<And2>()
            .register_device::<Nand2>()
            .register_device::<Or2>()
            .register_device::<Xor2>()
            .register_device::<Not>()
            .register_device::<Clock>()
            .register_device::<JKFlipFlop>()
            .register_device::<DFlipFlop>()
            .register_device::<TFlipFlop>()
            .register_device::<BinaryDisplay>()
            .register_device::<BinarySwitch>();

        app.add_systems(Update, tick_clocks)
            .add_systems(
                Update,
                (
                    toggle_binary_switch,
                    update_board_binary_displays
                        .chain()
                        .after(propagate_signals),
                ), //TODO: observers?
            )
            .add_systems(Update, update_device_positions);
    }
}
