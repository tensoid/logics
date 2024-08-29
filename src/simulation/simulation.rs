use std::collections::HashSet;

use bevy::prelude::*;
use moonshine_view::View;
use uuid::Uuid;

use crate::{
    designer::{
        board_binary_io::BoardBinaryOutputPin,
        board_entity::BoardEntityViewKind,
        chip::{BuiltinChip, ChipInputPin},
        pin::{PinModelCollection, PinView},
        signal_state::SignalState,
        wire::Wire,
    },
    get_model, get_model_mut,
};

/// Sets the [`SignalState`] of all floating (unconnected) destination pins to [`SignalState::Low`].
#[allow(clippy::type_complexity)]
pub fn handle_floating_pins(
    q_dest_pins: Query<(&PinView, Entity), Or<(With<ChipInputPin>, With<BoardBinaryOutputPin>)>>,
    q_wires: Query<&Wire>,
    q_parents: Query<&Parent>,
    q_board_entities: Query<&View<BoardEntityViewKind>>,
    mut q_chip_models: Query<&mut PinModelCollection>,
) {
    let dest_pin_uuids: HashSet<Uuid> = q_wires.iter().filter_map(|w| w.dest_pin_uuid).collect();

    for (pin_view, pin_entity) in q_dest_pins.iter() {
        if dest_pin_uuids.contains(&pin_view.uuid) {
            continue;
        }

        if let Some(mut pin_model_collection) =
            get_model_mut!(q_parents, q_board_entities, q_chip_models, pin_entity)
        {
            pin_model_collection
                .get_model_mut(pin_view.uuid)
                .unwrap()
                .signal_state = SignalState::Low;
        }
    }
}

/// Evaluates all builtin chips and updates their models accordingly.
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

/// Syncronizes the destination pin with the source pin for every [`Wire`].
pub fn update_signals(
    mut q_wires: Query<(&Wire, &mut SignalState)>,
    q_pin_views: Query<(&PinView, Entity)>,
    q_parents: Query<&Parent>,
    q_board_entities: Query<&View<BoardEntityViewKind>>,
    mut q_chip_models: Query<&mut PinModelCollection>,
) {
    for (wire, mut wire_signal_state) in q_wires.iter_mut() {
        let Some(wire_src_uuid) = wire.src_pin_uuid else {
            return;
        };

        let (src_pin_view, src_pin_entity) = q_pin_views
            .iter()
            .find(|(p, _)| p.uuid.eq(&wire_src_uuid))
            .unwrap();
        let Some(src_pin_model_collection) =
            get_model!(q_parents, q_board_entities, q_chip_models, src_pin_entity)
        else {
            return;
        };

        let src_pin_signal_state = src_pin_model_collection
            .get_model(src_pin_view.uuid)
            .unwrap()
            .signal_state;

        *wire_signal_state = src_pin_signal_state;

        let Some(wire_dest_uuid) = wire.dest_pin_uuid else {
            return;
        };

        let (dest_pin_view, dest_pin_entity) = q_pin_views
            .iter()
            .find(|(p, _)| p.uuid.eq(&wire_dest_uuid))
            .unwrap();
        let Some(mut dest_pin_model_collection) =
            get_model_mut!(q_parents, q_board_entities, q_chip_models, dest_pin_entity)
        else {
            return;
        };

        dest_pin_model_collection
            .get_model_mut(dest_pin_view.uuid)
            .unwrap()
            .signal_state = src_pin_signal_state;
    }
}
