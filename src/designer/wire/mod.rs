use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use moonshine_core::object::Object;
use moonshine_save::save::Save;
use moonshine_view::{BuildView, ViewCommands, Viewable};
use uuid::Uuid;

use crate::{get_cursor, get_cursor_mut, ui::cursor_captured::IsCursorCaptured};

use super::{
    bounding_box::BoundingBox,
    cursor::{Cursor, CursorState},
    devices::{
        binary_io::{BinaryDisplayPin, BinarySwitchPin},
        clock::ClockPin,
        generic_chip::{GenericChipInputPin, GenericChipOutputPin},
    },
    pin::PinView,
    render_settings::CircuitBoardRenderingSettings,
    signal_state::SignalState,
};

pub struct WirePlugin;

impl Plugin for WirePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            drag_wire.run_if(resource_equals(IsCursorCaptured(false))),
        )
        .add_systems(Update, update_wires);
    }
}

#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct Wire {
    pub src_pin_uuid: Option<Uuid>,
    pub dest_pin_uuid: Option<Uuid>,
}

#[derive(Bundle)]
pub struct WireBundle {
    pub wire: Wire,
    pub save: Save,
    pub signal_state: SignalState,
}

impl WireBundle {
    pub fn new(wire: Wire) -> Self {
        Self {
            wire,
            signal_state: SignalState::Low,
            save: Save,
        }
    }

    pub fn new_with_signal(wire: Wire, signal_state: SignalState) -> Self {
        Self {
            wire,
            signal_state,
            save: Save,
        }
    }
}

#[derive(Component)]
pub struct WireView;

#[derive(Bundle)]
pub struct WireViewBundle {
    wire_view: WireView,
    shape_bundle: ShapeBundle,
    stroke: Stroke,
}

impl WireViewBundle {
    pub fn new(render_settings: &CircuitBoardRenderingSettings) -> Self {
        Self {
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Line(Vec2::ZERO, Vec2::ZERO)),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.005),
                    ..default()
                },
                ..default()
            },
            stroke: Stroke::new(
                render_settings.signal_low_color,
                render_settings.wire_line_width,
            ),
            wire_view: WireView,
        }
    }
}

impl BuildView for Wire {
    fn build(world: &World, _: Object<Wire>, view: &mut ViewCommands<Wire>) {
        let render_settings = world.resource::<CircuitBoardRenderingSettings>();

        view.insert(WireViewBundle::new(render_settings));
    }
}

/**
 * Updates the wires location to always stay connected to its source and destination pins.
 * If the source or destination pin was deleted or the wire is just not connected this also deletes the wire entirely.
 */
//TODO: Optimisation potential with only updating necessary wires.
//TODO: maybe split into delete_dangling_wires and update_wires
//TODO: clean up the indents and ugly stuff
#[allow(clippy::type_complexity)]
pub fn update_wires(
    q_wires: Query<(&mut Wire, &Viewable<Wire>, Entity)>,
    q_dest_pins: Query<
        (&GlobalTransform, &PinView),
        Or<(With<GenericChipInputPin>, With<BinaryDisplayPin>)>,
    >,
    q_src_pins: Query<
        (&GlobalTransform, &PinView),
        Or<(
            With<GenericChipOutputPin>,
            With<BinarySwitchPin>,
            With<ClockPin>,
        )>,
    >,
    q_cursor: Query<(&Cursor, &Transform)>,
    mut q_wire_views: Query<&mut Path, With<WireView>>,
    mut commands: Commands,
) {
    let (cursor, cursor_transform) = get_cursor!(q_cursor);

    for (wire, wire_viewable, wire_entity) in q_wires.iter() {
        let Some(wire_src_pin_uuid) = wire.src_pin_uuid else {
            commands.entity(wire_entity).despawn();
            continue;
        };

        let mut wire_path = q_wire_views.get_mut(wire_viewable.view().entity()).unwrap();

        if let Some(wire_dest_pin_uuid) = wire.dest_pin_uuid {
            if let (Some((wire_src_pin_transform, _)), Some((wire_dest_pin_transform, _))) = (
                q_src_pins
                    .iter()
                    .find(|(_, p)| p.uuid.eq(&wire_src_pin_uuid)),
                q_dest_pins
                    .iter()
                    .find(|(_, p)| p.uuid.eq(&wire_dest_pin_uuid)),
            ) {
                let new_wire = shapes::Line(
                    wire_src_pin_transform.translation().truncate(),
                    wire_dest_pin_transform.translation().truncate(),
                );

                *wire_path = ShapePath::build_as(&new_wire);
            } else {
                commands.entity(wire_entity).despawn();
                continue;
            }
        } else if let CursorState::DraggingWire(dragged_wire) = cursor.state {
            //TODO: move this to drag_wire
            if dragged_wire.eq(&wire_entity) {
                if let Some((wire_src_pin_transform, _)) = q_src_pins
                    .iter()
                    .find(|(_, p)| p.uuid.eq(&wire_src_pin_uuid))
                {
                    let new_wire = shapes::Line(
                        wire_src_pin_transform.translation().truncate(),
                        cursor_transform.translation.truncate(),
                    );

                    *wire_path = ShapePath::build_as(&new_wire);
                }
            } else {
                commands.entity(wire_entity).despawn();
            }
        } else {
            commands.entity(wire_entity).despawn();
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn drag_wire(
    input: Res<ButtonInput<MouseButton>>,
    q_wire_src_pins: Query<
        (&BoundingBox, &PinView),
        Or<(
            With<GenericChipOutputPin>,
            With<BinarySwitchPin>,
            With<ClockPin>,
        )>,
    >,
    q_wire_dest_pins: Query<
        (&BoundingBox, &PinView),
        Or<(With<GenericChipInputPin>, With<BinaryDisplayPin>)>,
    >,
    mut q_wires: Query<&mut Wire>,
    mut q_cursor: Query<(&mut Cursor, &Transform), With<Cursor>>,
    mut commands: Commands,
) {
    let (mut cursor, cursor_transform) = get_cursor_mut!(q_cursor);

    if input.just_pressed(MouseButton::Left) && cursor.state == CursorState::Idle {
        for (bbox, pin_view) in q_wire_src_pins.iter() {
            if !bbox.point_in_bbox(cursor_transform.translation.truncate()) {
                continue;
            }

            // cursor is on pin
            let wire = commands
                .spawn(WireBundle::new(Wire {
                    src_pin_uuid: Some(pin_view.uuid),
                    dest_pin_uuid: None,
                }))
                .id();
            cursor.state = CursorState::DraggingWire(wire);
            return;
        }
    }

    if let CursorState::DraggingWire(wire_entity) = cursor.state {
        if let Ok(mut wire) = q_wires.get_mut(wire_entity) {
            if input.just_released(MouseButton::Left) {
                for (bbox, pin_view) in q_wire_dest_pins.iter() {
                    if bbox.point_in_bbox(cursor_transform.translation.truncate()) {
                        // connect wire to pin
                        wire.dest_pin_uuid = Some(pin_view.uuid);
                        cursor.state = CursorState::Idle;
                        return;
                    }
                }

                // dragged on nothing
                cursor.state = CursorState::Idle;
            }
        }
    }
}
