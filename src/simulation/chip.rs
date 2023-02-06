use bevy::prelude::*;

use super::expressions::Expr;

#[derive(Component)]
pub struct Chip;

#[derive(Component)]
pub struct ChipExtents(pub Vec2);

pub struct SpawnChipEvent {
    pub chip_name: String,
    pub position: Vec2,
}

#[derive(Resource)]
pub struct ChipSpecs(pub Vec<ChipSpec>);

#[derive(Component, Clone)]
pub struct ChipSpec {
    pub name: String,
    //pub expressions: Vec<Expr>,
    pub expression: Expr,
}

pub trait Evaluation {
    fn evaluate() -> bool;
}
