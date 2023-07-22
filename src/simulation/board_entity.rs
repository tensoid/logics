use bevy::prelude::Component;

/**
 * Marker component for all entities placable on the board.
 * Can be used for scene saving/loading for example.
 */
#[derive(Component)]
pub struct BoardEntity;
