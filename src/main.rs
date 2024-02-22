use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PrimaryWindow,
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
    pin::SpawnIOPinEvent,
    simulation::SimulationPlugin,
};
use ui::{circuit_board::CircuitBoardRenderingSettings, cursor::Cursor, ui::UIPlugin};

const WINDOW_TITLE: &str = "Logics";

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(Cursor::default())
        .insert_resource(ChipSpecs(vec![
            ChipSpec {
                name: "and".to_string(),
                //expressions: vec![Expr::from_string("0 & 1").unwrap()],
                expression: Expr::from_string("0 & 1").unwrap(),
            },
            ChipSpec {
                name: "or".to_string(),
                expression: Expr::from_string("0 | 1").unwrap(),
            },
            ChipSpec {
                name: "not".to_string(),
                expression: Expr::from_string("!0").unwrap(),
            },
            ChipSpec {
                name: "xor".to_string(),
                expression: Expr::from_string("(0 | 1) & !(0 & 1)").unwrap(),
            },
        ]))
        .insert_resource(CircuitBoardRenderingSettings {
            signal_low_color: Color::BLACK,
            signal_high_color: Color::GREEN,
            chip_pin_gap: 25.0,
            chip_pin_radius: 7.0,
            binary_io_pin_radius: 10.0,
            binary_io_handlebar_width: 8.0,
            binary_io_handlebar_length: 40.0,
            binary_io_handlebar_color: Color::BLACK,
            wire_line_width: 2.0,
        })
        .add_event::<SpawnChipEvent>()
        .add_event::<SpawnIOPinEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PanCamPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(SimulationPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_system(display_fps)
        .run();
}

fn display_fps(
    diagnostics: Res<Diagnostics>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = q_window.get_single_mut().unwrap();
    window.title = format!(
        "{} - {:.2}",
        WINDOW_TITLE,
        diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.average())
            .unwrap_or(0.0)
    );
}
