#[derive(PartialEq, Clone, Copy)]
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
}