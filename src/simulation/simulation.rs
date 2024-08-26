use bevy::prelude::*;
use moonshine_view::View;

use crate::designer::{
    board_binary_io::{BoardBinaryInputPin, BoardBinaryOutputPin},
    board_entity::BoardEntityViewKind,
    chip::{BuiltinChip, Chip, ChipInputPin, ChipOutputPin},
    pin::{PinModelCollection, PinView},
    signal_state::{self, SignalState},
    wire::Wire,
};

pub fn evaluate_builtin_chips(
    mut q_builtin_chip_models: Query<(&BuiltinChip, &mut PinModelCollection)>,
) {
    for (builtin_chip, mut pin_model_collection) in q_builtin_chip_models.iter_mut() {
        match builtin_chip.name.as_str() {
            "AND-2" => {
                pin_model_collection[2].signal_state = if pin_model_collection[0].signal_state
                    == SignalState::High
                    && pin_model_collection[1].signal_state == SignalState::High
                {
                    SignalState::High
                } else {
                    SignalState::Low
                };
            }
            "NAND-2" => {
                pin_model_collection[2].signal_state = if pin_model_collection[0].signal_state
                    == SignalState::High
                    && pin_model_collection[1].signal_state == SignalState::High
                {
                    SignalState::Low
                } else {
                    SignalState::High
                };
            }
            "OR-2" => {
                pin_model_collection[2].signal_state = if pin_model_collection[0].signal_state
                    == SignalState::High
                    || pin_model_collection[1].signal_state == SignalState::High
                {
                    SignalState::High
                } else {
                    SignalState::Low
                };
            }
            "NOT" => {
                pin_model_collection[1].signal_state = !pin_model_collection[0].signal_state;
            }
            "XOR-2" => {
                pin_model_collection[2].signal_state = if (pin_model_collection[0].signal_state
                    == SignalState::High
                    && pin_model_collection[1].signal_state == SignalState::Low)
                    || (pin_model_collection[0].signal_state == SignalState::Low
                        && pin_model_collection[1].signal_state == SignalState::High)
                {
                    SignalState::High
                } else {
                    SignalState::Low
                };
            }
            _ => panic!(
                "Tried to evaluate unknown BuiltinChip: {}",
                builtin_chip.name
            ),
        }
    }
}

pub fn update_signals(
    mut q_wires: Query<(&Wire, &mut SignalState), With<Wire>>,
    q_pin_views: Query<&PinView>,
    q_parents: Query<&Parent>,
    q_board_entities: Query<&View<BoardEntityViewKind>>,
    mut q_chip_models: Query<&mut PinModelCollection>,
) {
    for (wire, mut wire_signal_state) in q_wires.iter_mut() {
        let Some(wire_src_entity) = wire.src_pin else {
            return;
        };

        //TODO: make macro out of this (getting model from view) that returns Option
        let src_pin_view = q_pin_views.get(wire_src_entity).unwrap();
        let src_board_entity = q_parents.iter_ancestors(wire_src_entity).last().unwrap();
        let src_model_entity = q_board_entities
            .get(src_board_entity)
            .unwrap()
            .viewable()
            .entity();

        let src_pin_model_collection = q_chip_models.get(src_model_entity).unwrap();

        // update wire state
        // update dest pin state
    }
}
