use std::collections::HashSet;

use bevy::prelude::*;

use crate::events::events::SaveBoardEvent;

use super::{board_entity::BoardEntity, io_pin::IOPinState, wire::WireState};

#[derive()]
struct SaveData {
    wires: Vec<WireState>,
    io_pins: Vec<IOPinState>,
}

pub fn save_board() {}

pub fn load_board() {}
