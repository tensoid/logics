use bevy::prelude::*;

#[derive(Component, Clone, Reflect, Copy)]
#[reflect(Component)]
pub struct Rotation(pub f32);

//TODO: maybe make a spatial folder or model folder or smth

#[allow(dead_code)]
impl Rotation {
    pub const ZERO: Self = Rotation(0.0);

    pub fn new(radians: f32) -> Self {
        Self(radians)
    }
}

impl Default for Rotation {
    fn default() -> Self {
        Self::ZERO
    }
}
