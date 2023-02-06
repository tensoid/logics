use bevy::prelude::*;
use bevy_pancam::PanCam;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(PanCam {
        grab_buttons: vec![MouseButton::Middle],
        enabled: true,
        zoom_to_cursor: false,
        min_scale: 1.,
        max_scale: Some(40.),
        ..default()
    });
}
