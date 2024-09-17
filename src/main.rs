use bevy::{asset::AssetMetaCheck, prelude::*, window::PresentMode};
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
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
    pin::{PinModel, PinModelCollection},
    DesignerPlugins,
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

    // FIXME
    // Limit FPS as temorary workaround for has_event issue.
    // With FPS above 60, systems with the run condition has_event get executed multiple times
    // per event and this causes a crash in the moonshine_save crate upon saving or loading.
    app.add_plugins(FramepacePlugin);
    app.add_systems(Startup, |mut settings: ResMut<FramepaceSettings>| {
        settings.limiter = Limiter::from_framerate(60.0);
    });

    #[cfg(debug_assertions)]
    app.add_plugins(DebugPlugin);

    app.add_plugins((SavePlugin, LoadPlugin))
        .add_plugins(PanCamPlugin)
        .add_plugins(ShapePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DesignerPlugins)
        .add_plugins(EventsPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(UIPlugin);

    app.run();
}

fn limit_fps(mut settings: ResMut<FramepaceSettings>) {
    settings.limiter = Limiter::from_framerate(30.0);
}
