use bevy::prelude::*;

#[derive(Component)]
pub struct Chip;

#[derive(Component)]
pub struct ChipExtents(pub Vec2);

pub trait Evaluation {
    fn evaluate() -> bool;
}