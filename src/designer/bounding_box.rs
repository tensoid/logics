use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

#[derive(Clone)]
pub struct WireShape {
    pub points: Vec<Vec2>,
    pub wire_width: f32,
}

impl IntersectsVolume<Aabb2d> for WireShape {
    fn intersects(&self, aabb: &Aabb2d) -> bool {
        for window in self.points.windows(2) {
            if line_intersects_rect(window[0], window[1], aabb.min, aabb.max) {
                return true;
            }
        }

        false
    }
}

fn line_intersects_rect(line_start: Vec2, line_end: Vec2, rect_min: Vec2, rect_max: Vec2) -> bool {
    // Define the rectangle edges as line segments
    let rect_edges = [
        (rect_min, Vec2::new(rect_max.x, rect_min.y)), // Bottom edge
        (Vec2::new(rect_max.x, rect_min.y), rect_max), // Right edge
        (rect_max, Vec2::new(rect_min.x, rect_max.y)), // Top edge
        (Vec2::new(rect_min.x, rect_max.y), rect_min), // Left edge
    ];

    // Check if the line intersects any of the rectangle's edges
    for &(edge_start, edge_end) in &rect_edges {
        if line_segment_intersection(line_start, line_end, edge_start, edge_end) {
            return true;
        }
    }

    // Check if the line segment is completely inside the rectangle
    is_point_in_rect(line_start, rect_min, rect_max)
        || is_point_in_rect(line_end, rect_min, rect_max)
}

fn line_segment_intersection(p1: Vec2, p2: Vec2, q1: Vec2, q2: Vec2) -> bool {
    fn cross(v1: Vec2, v2: Vec2) -> f32 {
        v1.x * v2.y - v1.y * v2.x
    }

    let r = p2 - p1;
    let s = q2 - q1;
    let r_cross_s = cross(r, s);
    let q_minus_p = q1 - p1;
    let t = cross(q_minus_p, s) / r_cross_s;
    let u = cross(q_minus_p, r) / r_cross_s;

    // Check if r × s is not zero (lines are not parallel or collinear)
    // and t and u are between 0 and 1 (intersects within the segments)
    r_cross_s != 0.0 && t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0
}

fn is_point_in_rect(point: Vec2, rect_min: Vec2, rect_max: Vec2) -> bool {
    point.x >= rect_min.x && point.x <= rect_max.x && point.y >= rect_min.y && point.y <= rect_max.y
}

fn is_point_on_wire(point: Vec2, wire_points: Vec<Vec2>, wire_width: f32) -> bool {
    let half_width = wire_width / 2.0;

    for window in wire_points.windows(2) {
        if point_to_line_segment_distance(point, window[0], window[1]) <= half_width {
            return true;
        }
    }

    false
}

fn point_to_line_segment_distance(point: Vec2, start: Vec2, end: Vec2) -> f32 {
    let line_vec = end - start;
    let point_vec = point - start;
    let line_len = line_vec.length();

    // Project the point onto the line, clamping to the line segment
    let t = (point_vec.dot(line_vec) / line_len.powi(2)).clamp(0.0, 1.0);
    let projection = start + line_vec * t;

    // Return the distance from the point to the closest point on the line segment
    (point - projection).length()
}

#[derive(Clone)]
pub enum BoundingShape {
    Aabb(Aabb2d),
    Circle(BoundingCircle),
    Wire(WireShape),
}

//TODO: rework, maybe add onto model?
//TODO: split into plugin and files
#[derive(Component, Clone)]
pub struct BoundingBox {
    pub bounding_shape: BoundingShape,
    pub offset: Vec2,
    pub selectable: bool,
}

#[allow(dead_code)]
impl BoundingBox {
    pub fn rect_new(half_size: Vec2, selectable: bool) -> Self {
        BoundingBox {
            bounding_shape: BoundingShape::Aabb(Aabb2d::new(Vec2::ZERO, half_size)),
            offset: Vec2::ZERO,
            selectable,
        }
    }

    pub fn rect_with_offset(half_size: Vec2, offset: Vec2, selectable: bool) -> Self {
        BoundingBox {
            bounding_shape: BoundingShape::Aabb(Aabb2d::new(Vec2::ZERO, half_size)),
            offset,
            selectable,
        }
    }

    pub fn circle_with_offset(radius: f32, offset: Vec2, selectable: bool) -> Self {
        BoundingBox {
            bounding_shape: BoundingShape::Circle(BoundingCircle::new(Vec2::ZERO, radius)),
            offset,
            selectable,
        }
    }

    pub fn circle_new(radius: f32, selectable: bool) -> Self {
        BoundingBox {
            bounding_shape: BoundingShape::Circle(BoundingCircle::new(Vec2::ZERO, radius)),
            offset: Vec2::ZERO,
            selectable,
        }
    }

    pub fn wire_new(points: Vec<Vec2>, wire_width: f32, selectable: bool) -> Self {
        Self {
            bounding_shape: BoundingShape::Wire(WireShape { points, wire_width }),
            offset: Vec2::ZERO,
            selectable,
        }
    }

    pub fn point_in_bbox(&self, point: Vec2) -> bool {
        match &self.bounding_shape {
            BoundingShape::Aabb(aabb) => aabb.closest_point(point) == point,
            BoundingShape::Circle(circle) => circle.closest_point(point) == point,
            BoundingShape::Wire(wire) => {
                is_point_on_wire(point, wire.points.clone(), wire.wire_width)
            }
        }
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        match &self.bounding_shape {
            BoundingShape::Aabb(aabb) => match &other.bounding_shape {
                BoundingShape::Aabb(other_aabb) => aabb.intersects(other_aabb),
                BoundingShape::Circle(other_circle) => aabb.intersects(other_circle),
                BoundingShape::Wire(other_wire_shape) => other_wire_shape.intersects(aabb),
            },
            BoundingShape::Circle(circle) => match &other.bounding_shape {
                BoundingShape::Aabb(other_aabb) => circle.intersects(other_aabb),
                BoundingShape::Circle(other_circle) => circle.intersects(other_circle),
                BoundingShape::Wire(_) => todo!(),
            },
            BoundingShape::Wire(wire_shape) => match &other.bounding_shape {
                BoundingShape::Aabb(other_aabb) => wire_shape.intersects(other_aabb),
                _ => todo!(),
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
            _ => {}
        }
    }
}
