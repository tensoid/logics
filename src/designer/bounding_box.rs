use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

#[derive(Clone)]
pub enum BoundingShape {
    Aabb(Aabb2d),
    Circle(BoundingCircle),
}

#[derive(Component, Clone)]
pub struct BoundingBox {
    pub bounding_shape: BoundingShape,
    pub offset: Vec2,
    pub interactable: bool,
}

impl BoundingBox {
    pub fn rect_new(half_size: Vec2, interactable: bool) -> BoundingBox {
        BoundingBox {
            bounding_shape: BoundingShape::Aabb(Aabb2d::new(Vec2::ZERO, half_size)),
            offset: Vec2::ZERO,
            interactable,
        }
    }

    pub fn rect_with_offset(half_size: Vec2, offset: Vec2, interactable: bool) -> BoundingBox {
        BoundingBox {
            bounding_shape: BoundingShape::Aabb(Aabb2d::new(Vec2::ZERO, half_size)),
            offset,
            interactable,
        }
    }

    pub fn circle_with_offset(radius: f32, offset: Vec2, interactable: bool) -> BoundingBox {
        BoundingBox {
            bounding_shape: BoundingShape::Circle(BoundingCircle::new(Vec2::ZERO, radius)),
            offset,
            interactable,
        }
    }

    pub fn circle_new(radius: f32, interactable: bool) -> BoundingBox {
        BoundingBox {
            bounding_shape: BoundingShape::Circle(BoundingCircle::new(Vec2::ZERO, radius)),
            offset: Vec2::ZERO,
            interactable,
        }
    }

    pub fn point_in_bbox(&self, point: Vec2) -> bool {
        match self.bounding_shape {
            BoundingShape::Aabb(aabb) => aabb.closest_point(point) == point,
            BoundingShape::Circle(circle) => circle.closest_point(point) == point,
        }
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        match self.bounding_shape {
            BoundingShape::Aabb(aabb) => match other.bounding_shape {
                BoundingShape::Aabb(other_aabb) => aabb.intersects(&other_aabb),
                BoundingShape::Circle(other_circle) => aabb.intersects(&other_circle),
            },
            BoundingShape::Circle(circle) => match other.bounding_shape {
                BoundingShape::Aabb(other_aabb) => circle.intersects(&other_aabb),
                BoundingShape::Circle(other_circle) => circle.intersects(&other_circle),
            },
        }
    }
}

pub fn update_bounding_boxes(mut q_entities: Query<(&GlobalTransform, &mut BoundingBox)>) {
    for (entity_transform, mut bbox) in q_entities.iter_mut() {
        let offset = bbox.offset;

        match bbox.bounding_shape {
            BoundingShape::Aabb(ref mut aabb) => {
                *aabb = Aabb2d::new(
                    entity_transform.translation().truncate() + offset,
                    aabb.half_size(),
                );
            }
            BoundingShape::Circle(ref mut circle) => {
                *circle = BoundingCircle::new(
                    entity_transform.translation().truncate() + offset,
                    circle.radius(),
                )
            }
        }
    }
}
