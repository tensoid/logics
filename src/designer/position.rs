use bevy::prelude::*;

#[derive(Component, Clone, Reflect, Copy)]
#[reflect(Component)]
pub struct Position(pub Vec2);

//TODO: maybe make a spatial folder or model folder or smth

#[allow(dead_code)]
impl Position {
    pub const ZERO: Self = Position(Vec2::ZERO);

    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Self(translation.truncate())
    }

    pub fn to_translation(&self, z: f32) -> Vec3 {
        self.0.extend(z)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::ZERO
    }
}
