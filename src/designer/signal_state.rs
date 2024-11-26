use std::ops::Not;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use moonshine_view::Viewable;

use super::{
    render_settings::CircuitBoardRenderingSettings,
    wire::{Wire, WireView},
};

//TODO: maybe move prev, next signal state to here

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

#[allow(dead_code)]
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
