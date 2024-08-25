use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::{
    render_settings::CircuitBoardRenderingSettings,
    wire::Wire,
};

#[derive(PartialEq, Clone, Copy, Debug, Component, Reflect)]
pub enum SignalState {
    High,
    Low,
}

impl SignalState {
    pub fn as_bool(&self) -> bool {
        match &self {
            SignalState::High => true,
            SignalState::Low => false,
        }
    }

    pub fn toggle(&mut self) {
        *self = match *self {
            SignalState::High => SignalState::Low,
            SignalState::Low => SignalState::High,
        };
    }
}

//TODO: make faster by not updating colors that havent changed.
/**
 * Updates all colors that are bound to a signal, e.g. pins or wires.
 */
#[allow(clippy::type_complexity)]
pub fn update_signal_colors(
    mut q_wires: Query<(&mut Stroke, &SignalState), With<Wire>>,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
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
