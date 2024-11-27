use std::collections::{HashSet, VecDeque};

use bevy::prelude::*;
use uuid::Uuid;

use crate::designer::{
    devices::generic_chip::GenericChip,
    pin::PinModelCollection,
    signal_state::{Signal, SignalState},
    wire::{Wire, WireJoint, WireNode},
};

/// Evaluates all builtin chips and updates their models accordingly.
pub fn evaluate_builtin_chips(
    mut q_builtin_chip_models: Query<(&GenericChip, &mut PinModelCollection)>,
) {
    for (builtin_chip, mut pin_model_collection) in q_builtin_chip_models.iter_mut() {
        match builtin_chip.name.as_str() {
            "AND-2" => {
                let next_signal = match (
                    pin_model_collection["A"].signal_state.get_signal(),
                    pin_model_collection["B"].signal_state.get_signal(),
                ) {
                    (Signal::Conflict, _) | (_, Signal::Conflict) => Signal::Conflict,
                    (Signal::High, Signal::High) => Signal::High,
                    _ => Signal::Low,
                };

                pin_model_collection["Q"]
                    .signal_state
                    .set_signal(next_signal);
            }
            "NAND-2" => {
                let next_signal = match (
                    pin_model_collection["A"].signal_state.get_signal(),
                    pin_model_collection["B"].signal_state.get_signal(),
                ) {
                    (Signal::Conflict, _) | (_, Signal::Conflict) => Signal::Conflict,
                    (Signal::High, Signal::High) => Signal::Low,
                    _ => Signal::High,
                };

                pin_model_collection["Q"]
                    .signal_state
                    .set_signal(next_signal);
            }
            "OR-2" => {
                let next_signal = match (
                    pin_model_collection["A"].signal_state.get_signal(),
                    pin_model_collection["B"].signal_state.get_signal(),
                ) {
                    (Signal::Conflict, _) | (_, Signal::Conflict) => Signal::Conflict,
                    (Signal::Low, Signal::Low) => Signal::Low,
                    _ => Signal::High,
                };

                pin_model_collection["Q"]
                    .signal_state
                    .set_signal(next_signal);
            }
            "NOT" => {
                let current_signal = pin_model_collection["A"].signal_state.get_signal().clone();
                pin_model_collection["Q"]
                    .signal_state
                    .set_signal(current_signal.negate());
            }
            "XOR-2" => {
                let next_signal = match (
                    pin_model_collection["A"].signal_state.get_signal(),
                    pin_model_collection["B"].signal_state.get_signal(),
                ) {
                    (Signal::Conflict, _) | (_, Signal::Conflict) => Signal::Conflict,
                    (Signal::Low, Signal::High) => Signal::High,
                    (Signal::High, Signal::Low) => Signal::High,
                    _ => Signal::Low,
                };

                pin_model_collection["Q"]
                    .signal_state
                    .set_signal(next_signal);
            }
            "JK-FF" => {
                // High-Edge triggered
                if *pin_model_collection["C"].signal_state.get_previous_signal() != Signal::Low
                    || *pin_model_collection["C"].signal_state.get_signal() != Signal::High
                {
                    continue;
                }

                let current_output_signal =
                    pin_model_collection["Q"].signal_state.get_signal().clone();

                let next_signal = match (
                    pin_model_collection["J"].signal_state.get_signal(),
                    pin_model_collection["K"].signal_state.get_signal(),
                ) {
                    (Signal::Conflict, _) | (_, Signal::Conflict) => Signal::Conflict,
                    (Signal::Low, Signal::Low) => current_output_signal,
                    (Signal::Low, Signal::High) => Signal::Low,
                    (Signal::High, Signal::Low) => Signal::High,
                    (Signal::High, Signal::High) => current_output_signal.negate(),
                };

                pin_model_collection["Q"]
                    .signal_state
                    .set_signal(next_signal);
            }
            "D-FF" => {
                // High-Edge triggered
                if *pin_model_collection["C"].signal_state.get_previous_signal() != Signal::Low
                    || *pin_model_collection["C"].signal_state.get_signal() != Signal::High
                {
                    continue;
                }

                let next_signal = pin_model_collection["D"].signal_state.get_signal().clone();

                pin_model_collection["Q"]
                    .signal_state
                    .set_signal(next_signal);
            }
            "T-FF" => {
                // High-Edge triggered
                if *pin_model_collection["C"].signal_state.get_previous_signal() != Signal::Low
                    || *pin_model_collection["C"].signal_state.get_signal() != Signal::High
                {
                    continue;
                }

                let current_output_signal =
                    pin_model_collection["Q"].signal_state.get_signal().clone();

                let next_signal = match pin_model_collection["T"].signal_state.get_signal() {
                    Signal::Conflict => Signal::Conflict,
                    Signal::Low => current_output_signal,
                    Signal::High => current_output_signal.negate(),
                };

                pin_model_collection["Q"]
                    .signal_state
                    .set_signal(next_signal);
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
    Wire(Entity),
    Pin(Uuid),
    WireJoint(Entity),
}

/// Propagates signals starting from all output pins using BFS.
/// TODO: If different signals stack on a node, high will always be preferred.
/// This should probably result in a conflict signal state.
/// TODO: Optimize by only queueing changed output pins.
pub fn propagate_signals(
    mut q_wires: Query<(&Wire, &mut SignalState, Entity)>,
    mut q_pin_model_collections: Query<&mut PinModelCollection>,
    mut q_wire_joints: Query<&mut SignalState, (With<WireJoint>, Without<Wire>)>,
) {
    let mut visited: HashSet<SignalNode> = HashSet::new();
    let mut queue: VecDeque<SignalNode> = q_pin_model_collections
        .iter()
        .flat_map(|pin_model_collection| pin_model_collection.iter_outputs())
        .map(|pin_model| SignalNode::Pin(pin_model.uuid))
        .collect();

    while let Some(node) = queue.pop_front() {
        match &node {
            SignalNode::Pin(pin_uuid) => {
                let pin_model = PinModelCollection::find_in_collections(
                    *pin_uuid,
                    q_pin_model_collections.iter(),
                )
                .unwrap();

                for (wire, mut wire_signal_state, wire_entity) in q_wires.iter_mut() {
                    if !wire.nodes.iter().any(|node| matches!(node, WireNode::Pin(wire_node_pin_uuid) if wire_node_pin_uuid == pin_uuid)) {
                        continue;
                    }

                    wire_signal_state
                        .push_signal(pin_model.signal_state.get_latest_signal().clone());

                    enqueue_if_new(&mut queue, &visited, SignalNode::Wire(wire_entity));
                }
            }
            SignalNode::WireJoint(joint_entity) => {
                let wire_joint_signal_state = q_wire_joints.get(*joint_entity).unwrap();

                for (wire, mut wire_signal_state, wire_entity) in q_wires.iter_mut() {
                    if !wire.nodes.iter().any(|node| matches!(node, WireNode::Joint(wire_node_joint_entity) if wire_node_joint_entity == joint_entity)) {
                        continue;
                    }

                    wire_signal_state
                        .push_signal(wire_joint_signal_state.get_latest_signal().clone());

                    enqueue_if_new(&mut queue, &visited, SignalNode::Wire(wire_entity));
                }
            }
            SignalNode::Wire(wire_entity) => {
                let (wire, wire_signal_state, _) = q_wires.get(*wire_entity).unwrap();

                for node in wire.nodes.iter() {
                    match node {
                        WireNode::Joint(joint_entity) => {
                            let mut wire_joint_signal_state =
                                q_wire_joints.get_mut(*joint_entity).unwrap();
                            wire_joint_signal_state
                                .push_signal(wire_signal_state.get_latest_signal().clone());

                            queue.push_back(SignalNode::WireJoint(*joint_entity));
                        }
                        WireNode::Pin(pin_uuid) => {
                            PinModelCollection::pin_model_scope(
                                q_pin_model_collections.iter_mut(),
                                *pin_uuid,
                                |pin_model| {
                                    pin_model
                                        .signal_state
                                        .push_signal(wire_signal_state.get_latest_signal().clone());

                                    enqueue_if_new(
                                        &mut queue,
                                        &visited,
                                        SignalNode::Pin(*pin_uuid),
                                    );
                                },
                            );
                        }
                    }
                }
            }
        }

        visited.insert(node);
    }
}

/// Resolves the final signal state after propagation, by checking the collected signal states on every [`SignalNode`].
pub fn apply_signals(
    mut q_wires: Query<&mut SignalState, With<Wire>>,
    mut q_pin_model_collections: Query<&mut PinModelCollection>,
    mut q_wire_joints: Query<&mut SignalState, (With<WireJoint>, Without<Wire>)>,
) {
    for mut wire_signal_state in q_wires.iter_mut() {
        wire_signal_state.apply_signals();
    }

    for mut pin_model_collection in q_pin_model_collections.iter_mut() {
        for pin_model in pin_model_collection.iter_mut() {
            pin_model.signal_state.apply_signals();
        }
    }

    for mut wire_joint_signal_state in q_wire_joints.iter_mut() {
        wire_joint_signal_state.apply_signals();
    }
}

//TODO: improve or comment
fn enqueue_if_new(
    queue: &mut VecDeque<SignalNode>,
    visited: &HashSet<SignalNode>,
    node: SignalNode,
) {
    if !visited.contains(&node) {
        queue.push_back(node);
    }
}
