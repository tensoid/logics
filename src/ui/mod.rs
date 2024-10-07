use bevy::prelude::*;

use self::{
    chip_selector::{chip_selector_button_interact, spawn_chip_selector},
    cursor_captured::{check_cursor_captured, IsCursorCaptured},
};

pub mod chip_selector;
pub mod cursor_captured;
mod styles;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(IsCursorCaptured(false))
            .add_systems(Startup, spawn_chip_selector)
            .add_systems(Update, check_cursor_captured)
            .add_systems(Update, chip_selector_button_interact);
    }
}
