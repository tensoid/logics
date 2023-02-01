use bevy::prelude::*;
use super::circuit_board::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_chip_at_cursor)
            .add_system(drag_chip);
            //.add_system(update_cursor_state);
    }
}