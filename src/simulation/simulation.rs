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

//TODO: only check changed board input pins and changed chips
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
            (
                Without<BoardBinaryOutputPin>,
                Without<ChipOutputPin>,
                Without<BoardBinaryInputPin>,
            ),
        ),
    >,
    mut q_chip_output_pins: Query<
        (&mut ChipOutputPin, &mut PinState, Entity, &Children),
        (
            With<ChipOutputPin>,
            (
                Without<BoardBinaryOutputPin>,
                Without<BoardBinaryInputPin>,
                Without<ChipInputPin>,
            ),
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
    mut q_wires: Query<
        (&Wire, &mut Stroke),
        (
            With<Wire>,
            Without<BoardBinaryOutputPin>,
            Without<BoardBinaryInputPin>,
        ),
    >,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    //TODO: after this, find all chips not simulated yet (not flagged) and sim them; happens when they are not connected
    //TODO: add floating chips to for loop iterator, or flag all entities that are sim starts and get them
    // then check if next entity is chip

    let mut current_entity_option: Option<Entity>;
    for (_, input_pin_state, input_pin_entity, _) in q_board_input_pins.iter() {
        current_entity_option = Some(input_pin_entity);
        let mut signal_state = *input_pin_state;
        while let Some(current_entity) = current_entity_option {
            let next_wire: Option<Entity>;
            current_entity_option = None;

            //TODO: update to match statement maybe
            if let Ok(board_input_pin) = q_board_input_pins.get(current_entity) {
                next_wire = Some(*board_input_pin.3.first().unwrap());
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
                        .find(|e| q_chip_output_pins.get(**e).is_ok())
                        .unwrap();
                    let mut chip_output_pin =
                        q_chip_output_pins.get_mut(*chip_output_pin_entity).unwrap();

                    *chip_output_pin.1 = signal_state;

                    // todo
                    next_wire = Some(*chip_output_pin.3.first().unwrap());
                } else {
                    next_wire = None;
                }
            } else {
                panic!("Found unexpected entity.")
            }

            // Update and follow wire
            if let Some(wire_entity) = next_wire {
                let (wire, mut stroke) = q_wires.get_mut(wire_entity).unwrap();
                let new_stroke = Stroke::new(
                    match signal_state {
                        PinState::High => render_settings.signal_high_color,
                        PinState::Low => render_settings.signal_low_color,
                    },
                    render_settings.wire_line_width,
                );
                *stroke = new_stroke;
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
        (&mut Fill, &PinState, Option<&Children>),
        Or<(
            With<ChipInputPin>,
            With<ChipOutputPin>,
            With<BoardBinaryInputPin>,
            With<BoardBinaryOutputPin>,
        )>,
    >,
    mut q_wires: Query<&mut Stroke, With<Wire>>,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    for (mut pin_fill, pin_state, children) in q_pins.iter_mut() {
        let signal_fill = match pin_state {
            PinState::Low => Fill::color(render_settings.signal_low_color),
            PinState::High => Fill::color(render_settings.signal_high_color),
        };

        let signal_wire_stroke = match pin_state {
            PinState::Low => Stroke::new(
                render_settings.signal_low_color,
                render_settings.wire_line_width,
            ),
            PinState::High => Stroke::new(
                render_settings.signal_high_color,
                render_settings.wire_line_width,
            ),
        };

        *pin_fill = signal_fill;

        if let Some(children) = children {
            for &child in children.iter() {
                let wire_result = q_wires.get_mut(child);
                if let Ok(mut wire_stroke) = wire_result {
                    *wire_stroke = signal_wire_stroke;
                }
            }
        }
    }
}
