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
    chip::{BuiltinChip, BuiltinChipBundle, BuiltinChips},
    pin::{PinModel, PinModelCollection, PinType},
    signal_state::SignalState,
    DesignerPlugin,
};

use events::EventsPlugin;
use input::InputPlugin;
use moonshine_save::{load::LoadPlugin, save::SavePlugin};
use simulation::SimulationPlugin;
use ui::UIPlugin;
use uuid::Uuid;

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
        BuiltinChipBundle::new(
            "AND-2".into(),
            PinModelCollection(vec![
                PinModel::new_input("A".into()),
                PinModel::new_input("B".into()),
                PinModel::new_output("Y".into()),
            ]),
        ),
        BuiltinChipBundle::new(
            "NAND-2".into(),
            PinModelCollection(vec![
                PinModel::new_input("A".into()),
                PinModel::new_input("B".into()),
                PinModel::new_output("Y".into()),
            ]),
        ),
        BuiltinChipBundle::new(
            "OR-2".into(),
            PinModelCollection(vec![
                PinModel::new_input("A".into()),
                PinModel::new_input("B".into()),
                PinModel::new_output("Y".into()),
            ]),
        ),
        BuiltinChipBundle::new(
            "NOT".into(),
            PinModelCollection(vec![
                PinModel::new_input("A".into()),
                PinModel::new_output("Y".into()),
            ]),
        ),
        BuiltinChipBundle::new(
            "XOR-2".into(),
            PinModelCollection(vec![
                PinModel::new_input("A".into()),
                PinModel::new_input("B".into()),
                PinModel::new_output("Y".into()),
            ]),
        ),
    ]));

    app.run();
}
