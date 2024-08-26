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
    chip::{BuiltinChip, BuiltinChips},
    DesignerPlugin,
};

use events::EventsPlugin;
use input::InputPlugin;
use moonshine_save::{load::LoadPlugin, save::SavePlugin};
use simulation::SimulationPlugin;
use ui::UIPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#logics-canvas".into()),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
    );

    #[cfg(debug_assertions)]
    app.add_plugins(DebugPlugin);

    app.add_plugins((SavePlugin, LoadPlugin))
        .add_plugins(PanCamPlugin)
        .add_plugins(ShapePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DesignerPlugin)
        .add_plugins(EventsPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(UIPlugin);

    app.insert_resource(BuiltinChips(vec![
        BuiltinChip {
            name: "AND-2".to_string(),
            num_inputs: 2,
            num_outputs: 1,
        },
        BuiltinChip {
            name: "NAND-2".to_string(),
            num_inputs: 2,
            num_outputs: 1,
        },
        BuiltinChip {
            name: "OR-2".to_string(),
            num_inputs: 2,
            num_outputs: 1,
        },
        BuiltinChip {
            name: "NOT".to_string(),
            num_inputs: 1,
            num_outputs: 1,
        },
        BuiltinChip {
            name: "XOR-2".to_string(),
            num_inputs: 2,
            num_outputs: 1,
        },
    ]));

    app.run();
}
