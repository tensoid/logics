use bevy::prelude::*;

use self::chip_selector::{
    chip_selector_button_interact, despawn_chip_selector, spawn_chip_selector, spawn_empty_board_tooltip, toggle_chip_selector, update_emtpy_board_tooltip, ChipSelectorState
};

pub mod chip_selector;
mod styles;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ChipSelectorState>()
            .add_systems(Startup, spawn_empty_board_tooltip)
            .add_systems(Update, update_emtpy_board_tooltip)
            .add_systems(Update, toggle_chip_selector)
            .add_systems(Update, chip_selector_button_interact)
            .add_systems(OnEnter(ChipSelectorState::Open), spawn_chip_selector)
            .add_systems(OnExit(ChipSelectorState::Open), despawn_chip_selector);
    }
}
