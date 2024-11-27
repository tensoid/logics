use bevy::{
    math::{
        bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
        VectorSpace,
    },
    prelude::*,
};

#[derive(Clone)]
pub struct WireShape {
    points: Vec<Vec2>,
    wire_width: f32,
}

impl IntersectsVolume<Aabb2d> for WireShape {
    fn intersects(&self, aabb: &Aabb2d) -> bool {
        // Step 1: Expand the AABB to account for the wire's width (radius).
        let expanded_aabb = Aabb2d {
            min: aabb.min - Vec2::new(self.wire_width / 2.0, self.wire_width / 2.0),
            max: aabb.max + Vec2::new(self.wire_width / 2.0, self.wire_width / 2.0),
        };

        // Step 2: Check each segment of the wire against the expanded AABB.
        for window in self.points.windows(2) {
            let start = window[0];
            let end = window[1];

            if line_segment_intersects_aabb(start, end, &expanded_aabb) {
                return true;
            }
        }

        // Step 3: Check if any rounded corner (point on the wire) intersects the original AABB.
        for point in &self.points {
            if point_inside_aabb(*point, aabb) {
                return true;
            }
        }

        false
    }
}

// Helper: Check if a point is inside an AABB.
fn point_inside_aabb(point: Vec2, aabb: &Aabb2d) -> bool {
    point.x >= aabb.min.x && point.x <= aabb.max.x && point.y >= aabb.min.y && point.y <= aabb.max.y
}

// Helper: Check if a line segment intersects an AABB.
fn line_segment_intersects_aabb(start: Vec2, end: Vec2, aabb: &Aabb2d) -> bool {
    // Step 1: Check if either endpoint is inside the AABB.
    if point_inside_aabb(start, aabb) || point_inside_aabb(end, aabb) {
        return true;
    }

    // Step 2: Check if the line intersects any of the AABB's edges.
    let edges = [
        (
            Vec2::new(aabb.min.x, aabb.min.y),
            Vec2::new(aabb.max.x, aabb.min.y),
        ), // Bottom edge
        (
            Vec2::new(aabb.max.x, aabb.min.y),
            Vec2::new(aabb.max.x, aabb.max.y),
        ), // Right edge
        (
            Vec2::new(aabb.max.x, aabb.max.y),
            Vec2::new(aabb.min.x, aabb.max.y),
        ), // Top edge
        (
            Vec2::new(aabb.min.x, aabb.max.y),
            Vec2::new(aabb.min.x, aabb.min.y),
        ), // Left edge
    ];

    for (edge_start, edge_end) in edges {
        if line_segments_intersect(start, end, edge_start, edge_end) {
            return true;
        }
    }

    false
}

// Helper: Check if two line segments intersect.
fn line_segments_intersect(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2) -> bool {
    let d1 = (b2 - b1).perp_dot(a1 - b1);
    let d2 = (b2 - b1).perp_dot(a2 - b1);
    let d3 = (a2 - a1).perp_dot(b1 - a1);
    let d4 = (a2 - a1).perp_dot(b2 - a1);

    // Check if the segments straddle each other.
    if (d1 * d2 < 0.0) && (d3 * d4 < 0.0) {
        return true;
    }

    // Check for collinear overlap (special case).
    if d1 == 0.0 && point_on_segment(a1, b1, b2) {
        return true;
    }
    if d2 == 0.0 && point_on_segment(a2, b1, b2) {
        return true;
    }
    if d3 == 0.0 && point_on_segment(b1, a1, a2) {
        return true;
    }
    if d4 == 0.0 && point_on_segment(b2, a1, a2) {
        return true;
    }

    false
}

// Helper: Check if a point is on a line segment.
fn point_on_segment(point: Vec2, seg_start: Vec2, seg_end: Vec2) -> bool {
    let cross = (point - seg_start).perp_dot(seg_end - seg_start).abs();
    let dot = (point - seg_start).dot(seg_end - seg_start);
    let seg_len2 = (seg_end - seg_start).length_squared();

    cross < f32::EPSILON && dot >= 0.0 && dot <= seg_len2
}

#[derive(Clone)]
pub enum BoundingShape {
    Aabb(Aabb2d),
    Circle(BoundingCircle),
    Wire(WireShape),
}

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
        match self.bounding_shape {
            BoundingShape::Aabb(aabb) => aabb.closest_point(point) == point,
            BoundingShape::Circle(circle) => circle.closest_point(point) == point,
            BoundingShape::Wire(_) => todo!(),
        }
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        match &self.bounding_shape {
            BoundingShape::Aabb(aabb) => match &other.bounding_shape {
                BoundingShape::Aabb(other_aabb) => aabb.intersects(other_aabb),
                BoundingShape::Circle(other_circle) => aabb.intersects(other_circle),
                BoundingShape::Wire(other_wire_shape) => other_wire_shape.intersects(&aabb),
            },
            BoundingShape::Circle(circle) => match &other.bounding_shape {
                BoundingShape::Aabb(other_aabb) => circle.intersects(other_aabb),
                BoundingShape::Circle(other_circle) => circle.intersects(other_circle),
                BoundingShape::Wire(_) => todo!(),
            },
            BoundingShape::Wire(wire_shape) => match &other.bounding_shape {
                BoundingShape::Aabb(other_aabb) => wire_shape.intersects(&other_aabb),
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
