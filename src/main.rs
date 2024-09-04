use bevy::{asset::AssetMetaCheck, prelude::*, window::PresentMode};
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
    chip::{BuiltinChipBundle, BuiltinChips},
    pin::{PinModel, PinModelCollection},
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
                    present_mode: PresentMode::Fifo,
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
                PinModel::new_input("B".into(), Uuid::nil()),
                PinModel::new_input("A".into(), Uuid::nil()),
                PinModel::new_output("Q".into(), Uuid::nil()),
            ]),
        ),
        BuiltinChipBundle::new(
            "NAND-2".into(),
            PinModelCollection(vec![
                PinModel::new_input("B".into(), Uuid::nil()),
                PinModel::new_input("A".into(), Uuid::nil()),
                PinModel::new_output("Q".into(), Uuid::nil()),
            ]),
        ),
        BuiltinChipBundle::new(
            "OR-2".into(),
            PinModelCollection(vec![
                PinModel::new_input("B".into(), Uuid::nil()),
                PinModel::new_input("A".into(), Uuid::nil()),
                PinModel::new_output("Q".into(), Uuid::nil()),
            ]),
        ),
        BuiltinChipBundle::new(
            "NOT".into(),
            PinModelCollection(vec![
                PinModel::new_input("A".into(), Uuid::nil()),
                PinModel::new_output("Q".into(), Uuid::nil()),
            ]),
        ),
        BuiltinChipBundle::new(
            "XOR-2".into(),
            PinModelCollection(vec![
                PinModel::new_input("B".into(), Uuid::nil()),
                PinModel::new_input("A".into(), Uuid::nil()),
                PinModel::new_output("Q".into(), Uuid::nil()),
            ]),
        ),
        BuiltinChipBundle::new(
            "JK-FF".into(),
            PinModelCollection(vec![
                PinModel::new_input("K".into(), Uuid::nil()),
                PinModel::new_input("C".into(), Uuid::nil()),
                PinModel::new_input("J".into(), Uuid::nil()),
                PinModel::new_output("Q".into(), Uuid::nil()),
            ]),
        ),
        BuiltinChipBundle::new(
            "D-FF".into(),
            PinModelCollection(vec![
                PinModel::new_input("C".into(), Uuid::nil()),
                PinModel::new_input("D".into(), Uuid::nil()),
                PinModel::new_output("Q".into(), Uuid::nil()),
            ]),
        ),
        BuiltinChipBundle::new(
            "T-FF".into(),
            PinModelCollection(vec![
                PinModel::new_input("C".into(), Uuid::nil()),
                PinModel::new_input("T".into(), Uuid::nil()),
                PinModel::new_output("Q".into(), Uuid::nil()),
            ]),
        ),
    ]));

    app.run();
}
