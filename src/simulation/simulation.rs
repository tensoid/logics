use bevy::prelude::*;
use moonshine_view::View;

use crate::{
    designer::{
        board_entity::BoardEntityViewKind,
        chip::BuiltinChip,
        pin::{PinModelCollection, PinView},
        signal_state::SignalState,
        wire::Wire,
    },
    get_model, get_model_mut,
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

        let src_pin_view = q_pin_views.get(wire_src_entity).unwrap(); //TODO: crashes when two port ins are connected to one port out and are then deleted
        let Some(src_pin_model_collection) =
            get_model!(q_parents, q_board_entities, q_chip_models, wire_src_entity)
        else {
            return;
        };

        let src_pin_signal_state = src_pin_model_collection[src_pin_view.pin_index].signal_state;

        *wire_signal_state = src_pin_signal_state;

        let Some(wire_dest_entity) = wire.dest_pin else {
            return;
        };

        let dest_pin_view = q_pin_views.get(wire_dest_entity).unwrap(); //TODO: probalby crashes too at some point
        let Some(mut dest_pin_model_collection) =
            get_model_mut!(q_parents, q_board_entities, q_chip_models, wire_dest_entity)
        else {
            return;
        };

        dest_pin_model_collection[dest_pin_view.pin_index].signal_state = src_pin_signal_state;
    }
}

//TODO: update floating pins
