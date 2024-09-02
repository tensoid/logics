use std::ops::Not;

use bevy::{ecs::reflect, prelude::*};
use bevy_prototype_lyon::prelude::*;
use moonshine_view::Viewable;

use super::{
    board_entity::BoardEntityViewKind,
    render_settings::CircuitBoardRenderingSettings,
    wire::{Wire, WireView},
};

#[derive(PartialEq, Clone, Copy, Debug, Component, Reflect)]
#[reflect(Component)]
pub enum SignalState {
    High,
    Low,
}

impl Not for SignalState {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            SignalState::High => SignalState::Low,
            SignalState::Low => SignalState::High,
        }
    }
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
    q_wires: Query<(&Viewable<BoardEntityViewKind>, &SignalState), With<Wire>>,
    mut q_wire_views: Query<&mut Stroke, With<WireView>>,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    // Color Wires
    for (wire_viewable, signal_state) in q_wires.iter() {
        let mut wire_stroke = q_wire_views.get_mut(wire_viewable.view().entity()).unwrap();

        let signal_wire_stroke = Stroke::new(
            match signal_state {
                SignalState::Low => render_settings.signal_low_color,
                SignalState::High => render_settings.signal_high_color,
            },
            render_settings.wire_line_width,
        );

        *wire_stroke = signal_wire_stroke;
    }
}
