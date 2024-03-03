use bevy::{
    math::bounding::{Aabb2d, BoundingVolume},
    prelude::*,
};

#[derive(Component)]
pub struct BoundingBox {
    pub aabb: Aabb2d,
    pub offset: Vec2,
    pub interactable: bool,
}

impl BoundingBox {
    pub fn new(half_size: Vec2, interactable: bool) -> BoundingBox {
        BoundingBox {
            aabb: Aabb2d::new(Vec2::ZERO, half_size),
            offset: Vec2::ZERO,
            interactable,
        }
    }

    pub fn with_offset(half_size: Vec2, offset: Vec2, interactable: bool) -> BoundingBox {
        BoundingBox {
            aabb: Aabb2d::new(Vec2::ZERO, half_size),
            offset,
            interactable,
        }
    }

    pub fn point_in_bbox(&self, point: Vec2) -> bool {
        self.aabb.closest_point(point) == point
    }
}

pub fn update_bounding_boxes(mut q_entities: Query<(&GlobalTransform, &mut BoundingBox)>) {
    for (entity_transform, mut bbox) in q_entities.iter_mut() {
        bbox.aabb = Aabb2d::new(
            entity_transform.translation().truncate() + bbox.offset,
            bbox.aabb.half_size(),
        );
    }
}
