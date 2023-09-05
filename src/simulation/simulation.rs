use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{Fill, Stroke};

use crate::ui::circuit_board::CircuitBoardRenderingSettings;

use super::{
    chip::{Chip, ChipSpec},
    pin::{BoardBinaryInputPin, BoardBinaryOutputPin, ChipInputPin, ChipOutputPin},
    pin_state::PinState,
    wire::Wire,
};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_floating_pins)
            .add_system(tick_simulation)
            .add_system(update_signal_colors.after(tick_simulation))
            .add_system(
                reset_sim_flags
                    .before(handle_floating_pins)
                    .before(tick_simulation),
            );
    }
}

/**
 * Removes all the flags that the simulation set to know its state.
 */
fn reset_sim_flags(mut q_chip_input_pins: Query<&mut ChipInputPin>) {
    for mut pin in q_chip_input_pins.iter_mut() {
        pin.input_received = false;
    }
}

/**
 * Flags all floating pins i.e. pins that don´t have a wire attached to them with input received (low).
 */
fn handle_floating_pins(
    mut q_chip_input_pins: Query<(&mut ChipInputPin, &mut PinState, Entity)>,
    q_wires: Query<&Wire>,
) {
    for (mut pin, mut pin_state, pin_entity) in q_chip_input_pins.iter_mut() {
        if !q_wires
            .iter()
            .any(|w| w.dest_pin.is_some() && w.dest_pin.unwrap() == pin_entity)
        {
            pin.input_received = true;
            *pin_state = PinState::Low;
        }
    }
}

//TODO: LEFT OFF: Follow all wires in simulation
//TODO: only check changed board input pins and changed chips or rather board input pins
#[allow(clippy::type_complexity)]
fn tick_simulation(
    q_chips: Query<(&Children, &ChipSpec), With<Chip>>,
    q_board_input_pins: Query<
        (&BoardBinaryInputPin, &mut PinState, Entity, &Children),
        With<BoardBinaryInputPin>,
    >,
    mut q_chip_input_pins: Query<
        (&mut ChipInputPin, &mut PinState, &Parent, Entity),
        (
            With<ChipInputPin>,
            Without<BoardBinaryOutputPin>,
            Without<ChipOutputPin>,
            Without<BoardBinaryInputPin>,
        ),
    >,
    mut q_chip_output_pins: Query<
        (&mut ChipOutputPin, &mut PinState, Entity),
        (
            With<ChipOutputPin>,
            Without<BoardBinaryOutputPin>,
            Without<BoardBinaryInputPin>,
            Without<ChipInputPin>,
        ),
    >,
    mut q_board_output_pins: Query<
        (&mut BoardBinaryOutputPin, &mut PinState),
        (
            With<BoardBinaryOutputPin>,
            Without<ChipInputPin>,
            Without<ChipOutputPin>,
            Without<BoardBinaryInputPin>,
        ),
    >,
    q_wires: Query<
        &Wire,
        (
            With<Wire>,
            Without<BoardBinaryOutputPin>,
            Without<BoardBinaryInputPin>,
        ),
    >,
) {
    //TODO: after this, find all chips not simulated yet (not flagged) and sim them; happens when they are not connected
    //TODO: add floating chips to for loop iterator, or flag all entities that are sim starts and get them
    // then check if next entity is chip

    let mut current_entity_option: Option<Entity>;
    for (_, input_pin_state, input_pin_entity, _) in q_board_input_pins.iter() {
        current_entity_option = Some(input_pin_entity);
        let mut signal_state = *input_pin_state;
        while let Some(current_entity) = current_entity_option {
            let next_wire: Option<&Wire>;
            current_entity_option = None;

            //TODO: update to match statement maybe
            if let Ok(board_input_pin) = q_board_input_pins.get(current_entity) {
                next_wire = q_wires
                    .iter()
                    .find(|w| w.src_pin.unwrap().eq(&board_input_pin.2));
            } else if let Ok(board_output_pin) = q_board_output_pins.get_mut(current_entity) {
                let (_, mut bbo_pin_state) = board_output_pin;
                *bbo_pin_state = signal_state;
                break;
            } else if let Ok(chip_input_pin) = q_chip_input_pins.get_mut(current_entity) {
                let (mut pin, mut pin_state, parent, _) = chip_input_pin;

                // flag pin, set pinstate, set color
                pin.input_received = true;
                *pin_state = signal_state;

                let chip = q_chips.get(parent.get()).unwrap();

                let chip_input_pins: Vec<_> = chip
                    .0
                    .iter()
                    .filter_map(|e| q_chip_input_pins.get(*e).ok())
                    .collect();

                let inputs_satisfied = !chip_input_pins.iter().any(|p| !p.0.input_received);

                if inputs_satisfied {
                    // -> update output pin
                    // -> set next wire
                    let inputs: &Vec<bool> =
                        &chip_input_pins.iter().map(|p| p.1.as_bool()).collect();

                    let result = chip.1.expression.evaluate(inputs);

                    signal_state = match result {
                        true => PinState::High,
                        false => PinState::Low,
                    };

                    //TODO: expand for multiple outputs
                    // let chip_output_pins: Vec<_> = chip
                    //     .0
                    //     .iter()
                    //     .filter_map(|e| q_chip_output_pins.get(*e).ok())
                    //     .collect();

                    let chip_output_pin_entity = chip
                        .0
                        .iter()
                        .find(|e: &&Entity| q_chip_output_pins.get(**e).is_ok())
                        .unwrap();
                    let mut chip_output_pin =
                        q_chip_output_pins.get_mut(*chip_output_pin_entity).unwrap();

                    *chip_output_pin.1 = signal_state;

                    // todo
                    next_wire = q_wires
                        .iter()
                        .find(|w| w.src_pin.unwrap().eq(&chip_output_pin.2));
                } else {
                    next_wire = None;
                }
            } else {
                panic!("Found unexpected entity.")
            }

            // Update and follow wire
            if let Some(wire) = next_wire {
                current_entity_option = wire.dest_pin;
            }
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
        (&mut Fill, &PinState, Entity),
        Or<(
            With<ChipInputPin>,
            With<ChipOutputPin>,
            With<BoardBinaryInputPin>,
            With<BoardBinaryOutputPin>,
        )>,
    >,
    mut q_wires: Query<(&Wire, &mut Stroke)>,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    for (mut pin_fill, pin_state, pin_entity) in q_pins.iter_mut() {
        let (signal_fill, signal_wire_stroke) = match pin_state {
            PinState::Low => (
                Fill::color(render_settings.signal_low_color),
                Stroke::new(
                    render_settings.signal_low_color,
                    render_settings.wire_line_width,
                ),
            ),
            PinState::High => (
                Fill::color(render_settings.signal_high_color),
                Stroke::new(
                    render_settings.signal_high_color,
                    render_settings.wire_line_width,
                ),
            ),
        };

        *pin_fill = signal_fill;

        let connected_wire = q_wires
            .iter_mut()
            .find(|w| w.0.src_pin.unwrap().eq(&pin_entity));

        if let Some(mut wire) = connected_wire {
            *wire.1 = signal_wire_stroke;
        }
    }
}
