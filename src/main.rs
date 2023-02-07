use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pancam::PanCamPlugin;
use bevy_prototype_lyon::prelude::*;

mod camera;
mod simulation;
mod ui;

use camera::CameraPlugin;
use simulation::{
    chip::{ChipSpec, ChipSpecs, SpawnChipEvent},
    expressions::Expr,
    simulation::SimulationPlugin,
};
use ui::{circuit_board::CursorState, ui::UIPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        // .insert_resource(WindowDescriptor {
        //     title: "Logics".to_string(),
        //     ..Default::default()
        // })
        .insert_resource(CursorState::Idle)
        .insert_resource(ChipSpecs(vec![ChipSpec {
            name: "and".to_string(),
            //expressions: vec![Expr::from_string("0 & 1").unwrap()],
            expression: Expr::from_string("0 & 1").unwrap(),
        }]))
        .add_event::<SpawnChipEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PanCamPlugin::default())
        .add_plugin(ShapePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(SimulationPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(display_fps)
        .run();
}

fn display_fps(diagnostics: Res<Diagnostics>, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    window.set_title(format!(
        "Logics - {:.2}",
        diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.average())
            .unwrap_or(0.0)
    ));
}
