use bevy::prelude::Component;

#[derive(PartialEq, Clone, Copy, Debug, Component)]
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
