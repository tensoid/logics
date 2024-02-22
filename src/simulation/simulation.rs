use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{Fill, Stroke};

use crate::ui::circuit_board::CircuitBoardRenderingSettings;

use super::{
    chip::{Chip, ChipSpec},
    pin::{BoardBinaryInputPin, BoardBinaryOutputPin, ChipInputPin, ChipOutputPin},
    signal_state::SignalState,
    wire::Wire,
};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(tick_simulation)
            .add_system(update_signal_colors.after(tick_simulation));
    }
}

#[allow(clippy::type_complexity)]
fn tick_simulation(
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
                if !src_pin.eq(&input_pin_entity) {
                    continue;
                }
            }

            *wire_signal_state = *input_pin_state;
        }
    }

    for (_, mut output_pin_state, output_pin_entity) in q_board_output_pins.iter_mut() {
        *output_pin_state = SignalState::Low;

        for (wire, wire_signal_state) in q_wires.iter_mut() {
            if let Some(dest_pin) = wire.dest_pin {
                if !dest_pin.eq(&output_pin_entity) {
                    continue;
                }
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
                if !dest_pin.eq(&input_pin_entity) {
                    continue;
                }
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

        let result = chip_spec.expression.evaluate(inputs);

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
                if !src_pin.eq(&output_pin_entity) {
                    continue;
                }
            }

            *wire_signal_state = *output_pin_state;
        }
    }
}

//TODO: make faster by not updating colors that havent changed.
/**
 * Updates all colors that are bound to a signal, e.g. pins or wires.
 */
#[allow(clippy::type_complexity)]
fn update_signal_colors(
    mut q_pins: Query<
        (&mut Fill, &SignalState, Entity),
        Or<(
            With<ChipInputPin>,
            With<ChipOutputPin>,
            With<BoardBinaryInputPin>,
            With<BoardBinaryOutputPin>,
        )>,
    >,
    mut q_wires: Query<(&mut Stroke, &SignalState), With<Wire>>,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    // Color Pins
    for (mut pin_fill, pin_state, pin_entity) in q_pins.iter_mut() {
        let signal_fill = match pin_state {
            SignalState::Low => Fill::color(render_settings.signal_low_color),
            SignalState::High => Fill::color(render_settings.signal_high_color),
        };

        // Color Pins
        *pin_fill = signal_fill;
    }

    // Color Wires
    for (mut stroke, signal_state) in q_wires.iter_mut() {
        let signal_wire_stroke = match signal_state {
            SignalState::Low => Stroke::new(
                render_settings.signal_low_color,
                render_settings.wire_line_width,
            ),

            SignalState::High => Stroke::new(
                render_settings.signal_high_color,
                render_settings.wire_line_width,
            ),
        };

        *stroke = signal_wire_stroke;
    }
}
