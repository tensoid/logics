use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use bevy_pancam::PanCamPlugin;

mod ui;
mod simulation;
mod camera;

use ui::ui::UIPlugin;
use simulation::simulation::SimulationPlugin;
use camera::CameraPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        // .insert_resource(WindowDescriptor {
            //     title: "Logics".to_string(),
            //     ..Default::default()
            // })
        .add_plugins(DefaultPlugins)
        .add_plugin(PanCamPlugin::default())
        .add_plugin(ShapePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(SimulationPlugin)
        .add_plugin(UIPlugin)
        .run();
}