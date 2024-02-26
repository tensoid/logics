use bevy::{
    diagnostic::{Diagnostics, DiagnosticsStore, FrameTimeDiagnosticsPlugin},
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
    chip::{ChipSpec, ChipSpecs},
    cursor::Cursor,
    expressions::Expr,
    SimulationPlugin,
};

use ui::UIPlugin;

const WINDOW_TITLE: &str = "Logics";

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#logics-canvas".into()),
            ..default()
        }),
        ..default()
    }));

    app.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
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
        .add_plugins(PanCamPlugin)
        .add_plugins(ShapePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(SimulationPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Update, display_fps)
        .run();
}

fn display_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = q_window.get_single_mut().unwrap();
    window.title = format!(
        "{} - {:.2}",
        WINDOW_TITLE,
        diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.average())
            .unwrap_or(0.0)
    );
}
