use bevy::prelude::*;

pub fn screen_to_world_space(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    position: Vec2,
) -> Vec2 {
    let window_size = Vec2::new(window.width(), window.height());
    let ndc = (position / window_size) * 2.0 - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    ndc_to_world.project_point3(ndc.extend(-1.0)).truncate()
}
