use bevy::prelude::*;

pub fn screen_to_world_space(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    position: Vec2,
) -> Vec2 {
    camera
        .viewport_to_world(camera_transform, position)
        .map(|ray| ray.origin.truncate())
        .unwrap()
}
