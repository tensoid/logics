use bevy::prelude::*;

use crate::designer::{
    board_binary_io_pins::{BoardBinaryInputPin, BoardBinaryOutputPin},
    chip::{Chip, ChipInputPin, ChipOutputPin, ChipSpec},
    signal_state::SignalState,
    wire::Wire,
};

#[allow(clippy::type_complexity)]
pub fn tick_simulation(
    q_board_input_pins: Query<
        (&BoardBinaryInputPin, &SignalState, Entity),
        (
            With<BoardBinaryInputPin>,
            Without<ChipInputPin>,
            Without<ChipOutputPin>,
            Without<BoardBinaryOutputPin>,
            Without<Chip>,
            Without<Wire>,
        ),
    >,
    mut q_board_output_pins: Query<
        (&mut BoardBinaryOutputPin, &mut SignalState, Entity),
        (
            With<BoardBinaryOutputPin>,
            Without<ChipInputPin>,
            Without<ChipOutputPin>,
            Without<BoardBinaryInputPin>,
            Without<Chip>,
            Without<Wire>,
        ),
    >,
    q_chips: Query<
        (&Children, &ChipSpec),
        (
            With<Chip>,
            Without<ChipInputPin>,
            Without<ChipOutputPin>,
            Without<BoardBinaryInputPin>,
            Without<BoardBinaryOutputPin>,
            Without<Wire>,
        ),
    >,
    mut q_chip_input_pins: Query<
        (&mut ChipInputPin, &mut SignalState, &Parent, Entity),
        (
            With<ChipInputPin>,
            Without<BoardBinaryOutputPin>,
            Without<ChipOutputPin>,
            Without<BoardBinaryInputPin>,
            Without<Chip>,
            Without<Wire>,
        ),
    >,
    mut q_chip_output_pins: Query<
        (&mut ChipOutputPin, &mut SignalState, Entity),
        (
            With<ChipOutputPin>,
            Without<BoardBinaryOutputPin>,
            Without<BoardBinaryInputPin>,
            Without<ChipInputPin>,
            Without<Chip>,
            Without<Wire>,
        ),
    >,
    mut q_wires: Query<
        (&Wire, &mut SignalState),
        (
            With<Wire>,
            Without<BoardBinaryOutputPin>,
            Without<BoardBinaryInputPin>,
            Without<ChipInputPin>,
            Without<Chip>,
        ),
    >,
) {
    for (_, input_pin_state, input_pin_entity) in q_board_input_pins.iter() {
        for (wire, mut wire_signal_state) in q_wires.iter_mut() {
            if let Some(src_pin) = wire.src_pin {
                if src_pin != input_pin_entity {
                    continue;
                }
            } else {
                continue;
            }

            *wire_signal_state = *input_pin_state;
        }
    }

    for (_, mut output_pin_state, output_pin_entity) in q_board_output_pins.iter_mut() {
        *output_pin_state = SignalState::Low;

        for (wire, wire_signal_state) in q_wires.iter_mut() {
            if let Some(dest_pin) = wire.dest_pin {
                if dest_pin != output_pin_entity {
                    continue;
                }
            } else {
                continue;
            }

            *output_pin_state = *wire_signal_state;

            if wire_signal_state.eq(&SignalState::High) {
                break;
            }
        }
    }

    for (_, mut input_pin_state, _, input_pin_entity) in q_chip_input_pins.iter_mut() {
        *input_pin_state = SignalState::Low;

        for (wire, wire_signal_state) in q_wires.iter_mut() {
            if let Some(dest_pin) = wire.dest_pin {
                if dest_pin != input_pin_entity {
                    continue;
                }
            } else {
                continue;
            }

            *input_pin_state = *wire_signal_state;

            if wire_signal_state.eq(&SignalState::High) {
                break;
            }
        }
    }

    for (children, chip_spec) in q_chips.iter() {
        let chip_input_pins: Vec<_> = children
            .iter()
            .filter_map(|e| q_chip_input_pins.get(*e).ok())
            .collect();

        let inputs: &Vec<bool> = &chip_input_pins.iter().map(|p| p.1.as_bool()).collect();

        let result = true; //chip_spec.expression.evaluate(inputs);

        let result_signal_state = match result {
            true => SignalState::High,
            false => SignalState::Low,
        };

        let chip_output_pin_entity = children
            .iter()
            .find(|e: &&Entity| q_chip_output_pins.get(**e).is_ok())
            .unwrap();

        let mut chip_output_pin = q_chip_output_pins.get_mut(*chip_output_pin_entity).unwrap();

        *chip_output_pin.1 = result_signal_state;
    }

    for (_, output_pin_state, output_pin_entity) in q_chip_output_pins.iter() {
        for (wire, mut wire_signal_state) in q_wires.iter_mut() {
            if let Some(src_pin) = wire.src_pin {
                if src_pin != output_pin_entity {
                    continue;
                }
            } else {
                continue;
            }

            *wire_signal_state = *output_pin_state;
        }
    }
}
