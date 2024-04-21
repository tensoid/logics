use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_pancam::PanCamPlugin;
use bevy_prototype_lyon::prelude::*;

mod camera;
mod debug;
mod designer;
mod events;
mod input;
mod simulation;
mod ui;

use camera::CameraPlugin;
use debug::DebugPlugin;
use designer::{
    chip::{ChipSpec, ChipSpecs},
    DesignerPlugin,
};

use events::EventsPlugin;
use input::InputPlugin;
use simulation::{expressions::Expr, SimulationPlugin};
use ui::UIPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(AssetMetaCheck::Never);

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#logics-canvas".into()),
            present_mode: bevy::window::PresentMode::Immediate,
            ..default()
        }),
        ..default()
    }));

    #[cfg(debug_assertions)]
    app.add_plugins(DebugPlugin);

    app.insert_resource(ChipSpecs(vec![
        ChipSpec {
            name: "AND-2".to_string(),
            //expressions: vec![Expr::from_string("0 & 1").unwrap()],
            expression: Expr::from_string("0 & 1").unwrap(),
        },
        ChipSpec {
            name: "NAND-2".to_string(),
            //expressions: vec![Expr::from_string("0 & 1").unwrap()],
            expression: Expr::from_string("!(0 & 1)").unwrap(),
        },
        ChipSpec {
            name: "OR-2".to_string(),
            expression: Expr::from_string("0 | 1").unwrap(),
        },
        ChipSpec {
            name: "NOT".to_string(),
            expression: Expr::from_string("!0").unwrap(),
        },
        ChipSpec {
            name: "XOR-2".to_string(),
            expression: Expr::from_string("(0 | 1) & !(0 & 1)").unwrap(),
        },
    ]))
    .add_plugins(PanCamPlugin)
    .add_plugins(ShapePlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(DesignerPlugin)
    .add_plugins(EventsPlugin)
    .add_plugins(InputPlugin)
    .add_plugins(SimulationPlugin)
    .add_plugins(UIPlugin)
    .run();
}
