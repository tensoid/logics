// use bevy::{
//     diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
//     prelude::*,
//     window::PrimaryWindow,
// };

// const WINDOW_TITLE: &str = "Logics";

// /// Displays fps in window title.
// pub fn display_fps(
//     diagnostics: Res<DiagnosticsStore>,
//     mut q_window: Query<&mut Window, With<PrimaryWindow>>,
// ) {
//     let mut window = q_window.get_single_mut().unwrap();
//     window.title = format!(
//         "{} - {:.2}",
//         WINDOW_TITLE,
//         diagnostics
//             .get(&FrameTimeDiagnosticsPlugin::FPS)
//             .and_then(|fps| fps.average())
//             .unwrap_or(0.0)
//     );
// }
