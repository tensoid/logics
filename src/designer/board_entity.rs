use std::ops::{Deref, DerefMut};

use bevy::{math::VectorSpace, prelude::*};
use moonshine_core::{
    kind::Kind,
    spawn::{Spawn, WithChildren},
};
use moonshine_save::save::Save;

use crate::{events::events::SpawnBoardEntityEvent, get_cursor_mut};

use super::{
    bounding_box::BoundingBox,
    cursor::{Cursor, CursorState},
    render_settings::{self, CircuitBoardRenderingSettings},
    selection::{Selected, SelectionOutlineBundle},
};

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct Position(Vec2);

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
}

impl Deref for Position {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/**
 * Marker component for all entities placable on the board.
 * Can be used for scene saving/loading for example.
 */
#[derive(Component)]
pub struct BoardEntityView;

#[derive(Bundle)]
pub struct BoardEntityViewBundle {
    board_entity_view: BoardEntityView,
    bounding_box: BoundingBox,
    spatial_bundle: SpatialBundle,
}

impl BoardEntityViewBundle {
    pub fn new(position: Position, extents: Vec2) -> Self {
        Self {
            board_entity_view: BoardEntityView,
            bounding_box: BoundingBox::rect_with_offset(
                extents / Vec2::new(2.0, 2.0),
                Vec2::ZERO,
                true,
            ),
            spatial_bundle: SpatialBundle {
                transform: Transform::from_xyz(position.x, position.y, 0.0),
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct BoardEntityModel;

#[derive(Bundle)]
pub struct BoardEntityModelBundle {
    board_entity_model: BoardEntityModel,
    position: Position,
    save: Save,
}

impl BoardEntityModelBundle {
    pub fn new(position: Position) -> Self {
        Self {
            board_entity_model: BoardEntityModel,
            position,
            save: Save,
        }
    }
}

pub struct BoardEntityViewKind;

impl Kind for BoardEntityViewKind {
    type Filter = With<BoardEntityModel>;
}

// impl Spawn for BoardEntityBundle {
//     type Output = BoardEntityBundle;

//     fn spawn(&self, world: &World, entity: Entity) -> Self::Output {
//         todo!()
//     }
// }

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
