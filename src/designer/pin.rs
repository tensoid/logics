use super::signal_state::SignalState;
use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

pub enum PinType {
    Input,
    Output,
}

pub struct PinModel {
    pub signal_state: SignalState,
    pub pin_type: PinType,
    pub label: String,
}

#[derive(Component)]
pub struct PinModelCollection(pub Vec<PinModel>);

impl Deref for PinModelCollection {
    type Target = Vec<PinModel>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PinModelCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component)]
pub struct PinCollection;

#[derive(Component)]
pub struct PinView {
    pub pin_index: u32,
}

impl PinView {
    pub fn new(index: u32) -> Self {
        Self { pin_index: index }
    }
}
