use bevy::prelude::*;

#[derive(Component)]
pub struct ChipInputPin;

//TODO: split into files
#[derive(Component)]
pub struct ChipOutputPin;

#[derive(Component)]
pub struct BoardBinaryIOHandleBar;

//TODO: get rid of extents (also for ChipExtents)
#[derive(Component)]
pub struct BoardBinaryIOHandleBarExtents(pub Vec2);

#[derive(Component)]
pub struct BoardBinaryInput;

#[derive(Component)]
pub struct BoardBinaryInputPin;

#[derive(Component)]
pub struct BoardBinaryInputSwitch;

#[derive(Component)]
pub struct BoardBinaryOutput;

#[derive(Component)]
pub struct BoardBinaryOutputPin;

#[derive(Component)]
pub struct BoardBinaryOutputDisplay;

pub struct SpawnIOPinEvent {
    pub is_input: bool,
    pub position: Vec2,
}
