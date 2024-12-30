use bevy::prelude::*;
use bevy_pancam::{DirectionKeys, PanCam};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d).insert(PanCam {
        grab_buttons: vec![MouseButton::Middle],
        enabled: true,
        zoom_to_cursor: false,
        min_scale: 1.,
        max_scale: 40.0,
        move_keys: DirectionKeys::arrows(),
        ..default()
    });
}
