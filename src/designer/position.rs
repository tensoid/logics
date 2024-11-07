use bevy::prelude::*;

use std::ops::{Deref, DerefMut};

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct Position(pub Vec2);

#[allow(dead_code)]
impl Position {
    pub const ZERO: Self = Position(Vec2::ZERO);

    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
}

impl Deref for Position {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
