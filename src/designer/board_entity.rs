use bevy::prelude::*;

use crate::{events::events::SpawnBoardEntityEvent, get_cursor_mut};

use super::{
    cursor::{Cursor, CursorState},
    selection::Selected,
};

/**
 * Marker component for all entities placable on the board.
 * Can be used for scene saving/loading for example.
 */
#[derive(Component)]
pub struct BoardEntity;

/**
 * Handles task like correctly positioning the entity, or initializing a cursor drag.
 */
pub fn manage_additional_spawn_tasks(
    In(data_option): In<Option<(Entity, SpawnBoardEntityEvent)>>,
    mut q_cursor: Query<(Entity, &mut Cursor)>,
    mut commands: Commands,
    q_selected_entities: Query<Entity, With<Selected>>,
) {
    let Some((entity, spawn_ev)) = data_option else {
        return;
    };

    let (cursor_entity, mut cursor) = get_cursor_mut!(q_cursor);

    if spawn_ev.init_drag {
        for selected_entity in q_selected_entities.iter() {
            commands.entity(selected_entity).remove::<Selected>();
        }

        cursor.state = CursorState::Dragging;
        commands.entity(cursor_entity).add_child(entity);
        commands.entity(entity).insert(Selected);
    } else {
        commands.entity(entity).insert(Transform::from_xyz(
            spawn_ev.position.x,
            spawn_ev.position.y,
            0.0,
        ));
    }
}
