use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::{
    board_entity::BoardEntity, draw_layer::DrawLayer, events::SpawnIOPinEvent,
    render_settings::CircuitBoardRenderingSettings, signal_state::SignalState,
};

#[derive(Component)]
pub struct BoardBinaryIOHandleBar;

#[derive(Component)]
pub struct BoardBinaryIOHandleBarExtents(pub Vec2);

#[derive(Component)]
pub struct BoardBinaryInput;

#[derive(Component)]
pub struct BoardBinaryInputPin;

#[derive(Component)]
pub struct BoardBinaryInputSwitch;

#[derive(Component)]
pub struct BoardBinaryOutput;

#[derive(Component)]
pub struct BoardBinaryOutputPin;

#[derive(Component)]
pub struct BoardBinaryOutputDisplay;

pub fn spawn_io_pin_event(
    mut commands: Commands,
    mut spawn_ev: EventReader<SpawnIOPinEvent>,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    for ev in spawn_ev.read() {
        let pin_bundle = (
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: render_settings.binary_io_pin_radius,
                    ..default()
                }),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0.0, 0.0, DrawLayer::Pin.get_z()),
                    ..default()
                },
                ..default()
            },
            Fill::color(render_settings.signal_low_color),
        );

        let handle_bar_extents: Vec2 = Vec2::new(
            render_settings.binary_io_handlebar_length,
            render_settings.binary_io_handlebar_width,
        );

        let mut handle_bar_bundle = (
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    origin: RectangleOrigin::Center,
                    extents: handle_bar_extents,
                }),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0.0, 0.0, DrawLayer::HandleBar.get_z()),
                    ..default()
                },
                ..default()
            },
            BoardBinaryIOHandleBarExtents(handle_bar_extents),
            Fill::color(render_settings.binary_io_handlebar_color),
        );

        let switch_bundle = (
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: render_settings.binary_io_pin_radius,
                    ..default()
                }),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(
                        -render_settings.binary_io_handlebar_length,
                        0.0,
                        DrawLayer::Pin.get_z(),
                    ),
                    ..default()
                },
                ..default()
            },
            Fill::color(render_settings.binary_io_handlebar_color),
        );

        let display_bundle = (
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: render_settings.binary_io_pin_radius,
                    ..default()
                }),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(
                        render_settings.binary_io_handlebar_length,
                        0.0,
                        DrawLayer::Pin.get_z(),
                    ),
                    ..default()
                },
                ..default()
            },
            Fill::color(render_settings.signal_low_color),
        );

        let identity_spatial_bundle = SpatialBundle {
            transform: Transform::from_xyz(ev.position.x, ev.position.y, DrawLayer::Pin.get_z()),
            ..default()
        };

        if ev.is_input {
            handle_bar_bundle.0.spatial.transform.translation.x -=
                render_settings.binary_io_handlebar_length / 2.0;

            commands
                .spawn((BoardBinaryInput, identity_spatial_bundle, BoardEntity))
                .with_children(|parent| {
                    parent.spawn((pin_bundle, BoardBinaryInputPin, SignalState::Low));
                    parent.spawn((handle_bar_bundle, BoardBinaryIOHandleBar));
                    parent.spawn((switch_bundle, BoardBinaryInputSwitch));
                });
        } else {
            handle_bar_bundle.0.spatial.transform.translation.x +=
                render_settings.binary_io_handlebar_length / 2.0;

            commands
                .spawn((BoardBinaryOutput, identity_spatial_bundle, BoardEntity))
                .with_children(|parent| {
                    parent.spawn((pin_bundle, BoardBinaryOutputPin, SignalState::Low));
                    parent.spawn((handle_bar_bundle, BoardBinaryIOHandleBar));
                    parent.spawn((display_bundle, BoardBinaryOutputDisplay));
                });
        }
    }
}
