use bevy::{math::bounding::BoundingVolume, prelude::*};

use crate::designer::bounding_box::BoundingBox;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct BoundingBoxGizmos;

pub fn draw_bounding_boxes(
    mut bbox_gizmos: Gizmos<BoundingBoxGizmos>,
    q_bboxes: Query<&BoundingBox>,
) {
    for bbox in q_bboxes.iter() {
        let color = match bbox.interactable {
            false => Color::GREEN,
            true => Color::RED,
        };

        bbox_gizmos.rect_2d(bbox.aabb.center(), 0.0, bbox.aabb.half_size() * 2.0, color);
    }
}
