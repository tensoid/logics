use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{Fill, Stroke};

use crate::ui::circuit_board::CircuitBoardRenderingSettings;

use super::{
    chip::{Chip, ChipSpec},
    pin::{BoardInputPin, BoardOutputPin, ChipInputPin, ChipOutputPin},
    pin_state::PinState,
    wire::Wire,
};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(tick_simulation)
            .add_system(handle_floating_pins)
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
    mut q_chip_input_pins: Query<(&mut ChipInputPin, &mut Fill, Entity)>,
    q_wires: Query<&Wire>,
) {
    for (mut pin, mut pin_fill, pin_entity) in q_chip_input_pins.iter_mut() {
        if q_wires
            .iter()
            .find(|w| w.dest_pin.is_some() && w.dest_pin.unwrap() == pin_entity)
            .is_none()
        {
            pin.input_received = true;
            pin.pin_state = PinState::Low;
            *pin_fill = Fill::color(Color::RED);
        }
    }
}

//TODO: only check changed board input pins and changed chips
fn tick_simulation(
    q_chips: Query<(&Children, &ChipSpec), With<Chip>>,
    q_board_input_pins: Query<(&BoardInputPin, Entity, &Children), With<BoardInputPin>>,
    mut q_chip_input_pins: Query<
        (&mut ChipInputPin, &mut Fill, &Parent, Entity),
        (
            With<ChipInputPin>,
            (Without<BoardOutputPin>, Without<ChipOutputPin>),
        ),
    >,
    mut q_chip_output_pins: Query<
        (&mut ChipOutputPin, &mut Fill, Entity, &Children),
        (
            With<ChipOutputPin>,
            (Without<BoardOutputPin>, Without<ChipInputPin>),
        ),
    >,
    mut q_board_output_pins: Query<
        (&mut BoardOutputPin, &mut Fill),
        (With<BoardOutputPin>, Without<Wire>),
    >,
    mut q_wires: Query<(&Wire, &mut Stroke), (With<Wire>, Without<BoardOutputPin>)>,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    //TODO: after this, find all chips not simulated yet (not flagged) and sim them; happens when they are not connected
    //TODO: add floating chips to for loop iterator, or flag all entities that are sim starts and get them
    // then check if next entity is chip

    let mut current_entity_option: Option<Entity>;
    for (input_pin, input_pin_entity, input_pin_children) in q_board_input_pins.iter() {
        current_entity_option = Some(input_pin_entity);
        let mut pin_state = input_pin.0;
        while let Some(current_entity) = current_entity_option {
            let next_wire: Option<Entity>;
            current_entity_option = None;

            if let Ok(board_input_pin) = q_board_input_pins.get(current_entity) {
                next_wire = Some(*board_input_pin.2.first().unwrap());
            } else if let Ok(board_output_pin) = q_board_output_pins.get_mut(current_entity) {
                let (mut pin, mut fill) = board_output_pin;
                let new_fill = Fill::color(match pin_state {
                    PinState::High => Color::GREEN,
                    PinState::Low => Color::RED,
                });
                *fill = new_fill;
                pin.0 = pin_state;
                break;
            } else if let Ok(mut chip_input_pin) = q_chip_input_pins.get_mut(current_entity) {
                let (mut pin, mut pin_fill, _, _) = chip_input_pin;

                // flag pin, set pinstate, set color
                pin.input_received = true;
                pin.pin_state = pin_state;
                let new_fill = Fill::color(match pin_state {
                    PinState::High => Color::GREEN,
                    PinState::Low => Color::RED,
                });
                *pin_fill = new_fill;

                let parent_chip_entity = q_chip_input_pins.get(current_entity).unwrap().2.get();

                let chip = q_chips.get(parent_chip_entity).unwrap();
                let chip_input_pins: Vec<_> = chip
                    .0
                    .iter()
                    .filter_map(|e| q_chip_input_pins.get(*e).ok())
                    .collect();

                let inputs_satisfied = chip_input_pins
                    .iter()
                    .find(|p| !p.0.input_received)
                    .is_none();

                if inputs_satisfied {
                    // -> update output pin
                    // -> set next wire
                    let inputs: &Vec<bool> = &chip_input_pins
                        .iter()
                        .map(|p| p.0.pin_state.as_bool())
                        .collect();

                    let result = chip.1.expression.evaluate(inputs);

                    pin_state = match result {
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

                    *chip_output_pin.1 = Fill::color(match pin_state {
                        PinState::High => Color::GREEN,
                        PinState::Low => Color::RED,
                    });

                    // todo
                    next_wire = Some(*chip_output_pin.3.first().unwrap());
                } else {
                    next_wire = None;
                }
            } else {
                panic!("Found unexpected component.")
            }

            // Update and follow wire
            if let Some(wire_entity) = next_wire {
                let (wire, mut stroke) = q_wires.get_mut(wire_entity).unwrap();
                let new_stroke = Stroke::new(
                    match pin_state {
                        PinState::High => Color::GREEN,
                        PinState::Low => Color::RED,
                    },
                    render_settings.wire_line_width,
                );
                *stroke = new_stroke;
                current_entity_option = wire.dest_pin;
            }
        }
    }
}

fn update_pin_colors() {
    //TODO: extract pin color logic
}
