use bevy::prelude::Component;

#[derive(PartialEq, Clone, Copy, Debug, Component)]
pub enum PinState {
    High,
    Low,
}

impl PinState {
    pub fn as_bool(&self) -> bool {
        match &self {
            PinState::High => true,
            PinState::Low => false,
        }
    }

    pub fn toggle(&mut self) {
        *self = match *self {
            PinState::High => PinState::Low,
            PinState::Low => PinState::High,
        };
    }
}
