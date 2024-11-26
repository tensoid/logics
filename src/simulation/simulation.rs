use std::collections::{HashMap, HashSet, VecDeque};

use bevy::prelude::*;
use moonshine_view::View;
use uuid::Uuid;

use crate::designer::{
    devices::{device::DeviceViewKind, generic_chip::GenericChip},
    pin::{PinModelCollection, PinView},
    signal_state::SignalState,
    wire::{Wire, WireNode},
};

/// Sets the [`SignalState`] of all input pins to [`SignalState::Low`] to prepare for update signals.
#[allow(clippy::type_complexity)]
pub fn reset_input_pins(mut q_chip_models: Query<&mut PinModelCollection>) {
    for mut pin_model_collection in q_chip_models.iter_mut() {
        for pin_model in pin_model_collection.iter_inputs_mut() {
            pin_model.next_signal_state = SignalState::Low
        }
    }
}

/// Evaluates all builtin chips and updates their models accordingly.
pub fn evaluate_builtin_chips(
    mut q_builtin_chip_models: Query<(&GenericChip, &mut PinModelCollection)>,
) {
    for (builtin_chip, mut pin_model_collection) in q_builtin_chip_models.iter_mut() {
        match builtin_chip.name.as_str() {
            "AND-2" => {
                pin_model_collection["Q"].next_signal_state = match (
                    pin_model_collection["A"].signal_state,
                    pin_model_collection["B"].signal_state,
                ) {
                    (SignalState::High, SignalState::High) => SignalState::High,
                    _ => SignalState::Low,
                };
            }
            "NAND-2" => {
                pin_model_collection["Q"].next_signal_state = match (
                    pin_model_collection["A"].signal_state,
                    pin_model_collection["B"].signal_state,
                ) {
                    (SignalState::High, SignalState::High) => SignalState::Low,
                    _ => SignalState::High,
                };
            }
            "OR-2" => {
                pin_model_collection["Q"].next_signal_state = match (
                    pin_model_collection["A"].signal_state,
                    pin_model_collection["B"].signal_state,
                ) {
                    (SignalState::Low, SignalState::Low) => SignalState::Low,
                    _ => SignalState::High,
                };
            }
            "NOT" => {
                pin_model_collection["Q"].next_signal_state =
                    !pin_model_collection["A"].signal_state;
            }
            "XOR-2" => {
                pin_model_collection["Q"].next_signal_state = match (
                    pin_model_collection["A"].signal_state,
                    pin_model_collection["B"].signal_state,
                ) {
                    (SignalState::Low, SignalState::High) => SignalState::High,
                    (SignalState::High, SignalState::Low) => SignalState::High,
                    _ => SignalState::Low,
                };
            }
            "JK-FF" => {
                // High-Edge triggered
                if pin_model_collection["C"].previous_signal_state != SignalState::Low
                    || pin_model_collection["C"].signal_state != SignalState::High
                {
                    continue;
                }

                pin_model_collection["Q"].next_signal_state = match (
                    pin_model_collection["J"].signal_state,
                    pin_model_collection["K"].signal_state,
                ) {
                    (SignalState::Low, SignalState::Low) => pin_model_collection["Q"].signal_state,
                    (SignalState::Low, SignalState::High) => SignalState::Low,
                    (SignalState::High, SignalState::Low) => SignalState::High,
                    (SignalState::High, SignalState::High) => {
                        !pin_model_collection["Q"].signal_state
                    }
                }
            }
            "D-FF" => {
                // High-Edge triggered
                if pin_model_collection["C"].previous_signal_state != SignalState::Low
                    || pin_model_collection["C"].signal_state != SignalState::High
                {
                    continue;
                }

                pin_model_collection["Q"].next_signal_state =
                    pin_model_collection["D"].signal_state;
            }
            "T-FF" => {
                // High-Edge triggered
                if pin_model_collection["C"].previous_signal_state != SignalState::Low
                    || pin_model_collection["C"].signal_state != SignalState::High
                {
                    continue;
                }

                pin_model_collection["Q"].next_signal_state =
                    match pin_model_collection["T"].signal_state {
                        SignalState::Low => pin_model_collection["Q"].signal_state,
                        SignalState::High => !pin_model_collection["Q"].signal_state,
                    }
            }
            _ => panic!(
                "Tried to evaluate unknown BuiltinChip: {}",
                builtin_chip.name
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SignalNode {
    Wire(Wire),
    Pin(Uuid),
    WireJoint(Entity),
}

//TODO: split into fns (process_a, process_b) maybe
pub fn propagate_signals(
    mut q_wires: Query<(&Wire, &mut SignalState)>,
    q_pin_views: Query<(&PinView, Entity)>,
    q_parents: Query<&Parent>,
    q_board_entities: Query<&View<DeviceViewKind>>,
    mut q_chip_models: Query<&mut PinModelCollection>,
) {
    let mut visited: HashSet<SignalNode> = HashSet::new();
    let mut queue: VecDeque<SignalNode> = VecDeque::new();

    let mut pin_map: HashMap<Uuid, Vec<Wire>> = HashMap::new();
    let mut joint_map: HashMap<Entity, Vec<Wire>> = HashMap::new();

    while let Some(node) = queue.pop_front() {
        if visited.contains(&node) {
            continue;
        }

        match &node {
            SignalNode::Pin(pin_uuid) => {}
            SignalNode::Wire(wire) => {}
            SignalNode::WireJoint(joint_entity) => {}
        }

        visited.insert(node);
    }

    // // Update wires from output pins (might not need to clear pins?)
    // for (wire, mut wire_signal_state) in q_wires.iter_mut() {
    //     let pin_wire_nodes: Vec<Uuid> = wire
    //         .nodes
    //         .iter()
    //         .filter_map(|n| match n {
    //             WireNode::Pin(uuid) => Some(*uuid),
    //             WireNode::Joint(_) => None,
    //         })
    //         .collect();

    //     let new_wire_state = q_chip_models
    //         .iter()
    //         .flat_map(|pin_model_collection| pin_model_collection.0.clone())
    //         .filter(|pin_model| pin_wire_nodes.contains(&pin_model.uuid))
    //         .map(|pin_model| pin_model.signal_state)
    //         .any(|signal_state| signal_state == SignalState::High);

    //     *wire_signal_state = match new_wire_state {
    //         true => SignalState::High,
    //         false => SignalState::Low,
    //     };
    // }

    // update wire points by gathering all connected wires

    // Update wires from points
    // Update pins from wires

    // for (wire, mut wire_signal_state) in q_wires.iter_mut() {
    //     let Some(wire_src_uuid) = wire.src_pin_uuid else {
    //         continue;
    //     };

    //     let Some((src_pin_view, src_pin_entity)) =
    //         q_pin_views.iter().find(|(p, _)| p.uuid.eq(&wire_src_uuid))
    //     else {
    //         // this happens when the wire already spawned,
    //         // but the view for the device has not yet
    //         continue;
    //     };
    //     let Some(src_pin_model_collection) =
    //         get_model!(q_parents, q_board_entities, q_chip_models, src_pin_entity)
    //     else {
    //         continue;
    //     };

    //     let src_pin_signal_state = src_pin_model_collection
    //         .get_model(src_pin_view.uuid)
    //         .unwrap()
    //         .signal_state;

    //     *wire_signal_state = src_pin_signal_state;

    //     if src_pin_signal_state == SignalState::Low {
    //         continue;
    //     }

    //     let Some(wire_dest_uuid) = wire.dest_pin_uuid else {
    //         continue;
    //     };

    //     let (dest_pin_view, dest_pin_entity) = q_pin_views
    //         .iter()
    //         .find(|(p, _)| p.uuid.eq(&wire_dest_uuid))
    //         .unwrap();
    //     let Some(mut dest_pin_model_collection) =
    //         get_model_mut!(q_parents, q_board_entities, q_chip_models, dest_pin_entity)
    //     else {
    //         continue;
    //     };

    //     dest_pin_model_collection
    //         .get_model_mut(dest_pin_view.uuid)
    //         .unwrap()
    //         .next_signal_state = SignalState::High;
    // }
}
