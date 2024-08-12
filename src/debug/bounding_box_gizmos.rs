use bevy::{
    color::palettes::css::{LIME, RED},
    math::bounding::BoundingVolume,
    prelude::*,
};

use crate::designer::bounding_box::{BoundingBox, BoundingShape};

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct BoundingBoxGizmos;

pub fn draw_bounding_boxes(
    mut bbox_gizmos: Gizmos<BoundingBoxGizmos>,
    q_bboxes: Query<&BoundingBox>,
) {
    for bbox in q_bboxes.iter() {
        let color = match bbox.selectable {
            false => LIME,
            true => RED,
        };

        match bbox.bounding_shape {
            BoundingShape::Aabb(aabb) => {
                bbox_gizmos.rect_2d(aabb.center(), 0.0, aabb.half_size() * 2.0, color);
            }
            BoundingShape::Circle(circle) => {
                bbox_gizmos.circle_2d(circle.center(), circle.radius(), color);
            }
        }
    }
}
