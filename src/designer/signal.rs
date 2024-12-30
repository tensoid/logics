use bevy::prelude::*;

#[derive(PartialEq, Clone, Debug, Reflect)]
pub enum Signal {
    Low,
    High,
    Conflict,
}

#[allow(dead_code)]
impl Signal {
    /// Negates the signal if its Low or High, otherwise keeps original signal.
    pub fn negate(self) -> Self {
        match self {
            Signal::High => Signal::Low,
            Signal::Low => Signal::High,
            Signal::Conflict => Signal::Conflict,
        }
    }
}

#[derive(PartialEq, Clone, Debug, Component, Reflect)]
#[reflect(Component)]
pub struct SignalState {
    previous_signal: Signal,
    signal: Signal,
    next_signals: Vec<Signal>,
}

impl SignalState {
    pub fn new(signal: Signal) -> Self {
        Self {
            previous_signal: signal.clone(),
            signal,
            next_signals: Vec::new(),
        }
    }

    pub fn get_previous_signal(&self) -> &Signal {
        &self.previous_signal
    }

    pub fn get_signal(&self) -> &Signal {
        &self.signal
    }

    pub fn get_latest_signal(&self) -> &Signal {
        self.next_signals.last().unwrap_or(&self.signal)
    }

    pub fn set_signal(&mut self, signal: Signal) {
        self.next_signals.clear();
        self.next_signals.push(signal);
        self.apply_signals();
    }

    pub fn push_signal(&mut self, signal: Signal) {
        self.next_signals.push(signal);
    }

    pub fn apply_signals(&mut self) {
        if self.next_signals.is_empty() {
            return;
        }

        self.previous_signal = self.signal.clone();

        let conflict = !self
            .next_signals
            .iter()
            .all(|signal| signal == self.next_signals.first().unwrap());

        self.signal = match conflict {
            false => self.next_signals.first().unwrap().clone(),
            true => Signal::Conflict,
        };

        self.next_signals.clear();
    }
}
