use std::ops::{Deref, DerefMut};

use bevy::prelude::*;
use moonshine_core::
    kind::Kind
;
use moonshine_save::save::Save;
use moonshine_view::Viewable;


use super::
    bounding_box::BoundingBox
;

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct Position(pub Vec2);

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

pub fn update_board_entity_position(
    board_entities: Query<(&Viewable<BoardEntityViewKind>, &Position), Changed<Position>>,
    mut transform: Query<&mut Transform>,
) {
    for (viewable, position) in board_entities.iter() {
        let view = viewable.view();
        let mut transform = transform.get_mut(view.entity()).unwrap();
        *transform = Transform::from_translation(position.extend(transform.translation.z))
    }
}
