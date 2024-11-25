use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use moonshine_core::object::Object;
use moonshine_save::save::Save;
use moonshine_view::{BuildView, ViewCommands, Viewable};
use uuid::Uuid;

use crate::{
    get_cursor, get_cursor_mut, simulation::simulation::update_signals,
    ui::cursor_captured::IsCursorCaptured,
};

use super::{
    bounding_box::BoundingBox,
    cursor::{Cursor, CursorState},
    devices::{
        binary_io::{BinaryDisplayPin, BinarySwitchPin},
        clock::ClockPin,
        generic_chip::{GenericChipInputPin, GenericChipOutputPin},
    },
    pin::PinView,
    position::Position,
    render_settings::CircuitBoardRenderingSettings,
    signal_state::SignalState,
};

pub struct WirePlugin;

impl Plugin for WirePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (drag_wire, create_wire_point)
                .chain()
                .run_if(resource_equals(IsCursorCaptured(false))),
        )
        .add_systems(
            Update,
            (cancel_drag, despawn_dangling_wires, update_wires).chain(),
        )
        .add_systems(Update, update_wire_signal_colors.after(update_signals)); //TODO: observers or Changed<> Filter
    }
}

#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct Wire {
    pub src_pin_uuid: Option<Uuid>,
    pub dest_pin_uuid: Option<Uuid>,
    //TODO: maybe make children instead? on model or on view?
    pub wire_points: Vec<Entity>,
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

pub fn despawn_dangling_wires(
    mut commands: Commands,
    q_cursor: Query<&Cursor>,
    q_wires: Query<(&Wire, Entity)>,
    q_pins: Query<&PinView>,
) {
    let cursor = get_cursor!(q_cursor);

    for (wire, wire_entity) in q_wires.iter() {
        let is_dangling = wire.src_pin_uuid.is_none() || wire.dest_pin_uuid.is_none();
        let is_currently_dragged =
            matches!(cursor.state, CursorState::DraggingWire(w) if w == wire_entity);

        if is_dangling && !is_currently_dragged {
            commands.entity(wire_entity).despawn_recursive();
            continue;
        }

        let src_pin_exists = wire
            .src_pin_uuid
            .map(|uuid| q_pins.iter().any(|pin| pin.uuid == uuid))
            .unwrap_or(false);

        let dest_pin_exists = wire
            .dest_pin_uuid
            .map(|uuid| q_pins.iter().any(|pin| pin.uuid == uuid))
            .unwrap_or(false);

        if !src_pin_exists || !dest_pin_exists {
            commands.entity(wire_entity).despawn_recursive();
        }
    }
}

/// Updates the wires location to always stay connected to its source and destination pins.
/// Assumes that each wire has a src and dest connection or has a src and is being dragged.
//TODO: Optimisation potential with only updating necessary wires.
//TODO: clean up the indents and ugly stuff
#[allow(clippy::type_complexity)]
pub fn update_wires(
    q_wires: Query<(&mut Wire, &Viewable<Wire>)>,
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
    q_cursor: Query<&Transform, With<Cursor>>,
    mut q_wire_views: Query<&mut Path, With<WireView>>,
) {
    let cursor_transform = get_cursor!(q_cursor);

    for (wire, wire_viewable) in q_wires.iter() {
        let mut wire_path = q_wire_views.get_mut(wire_viewable.view().entity()).unwrap();

        let wire_src_pin_uuid = wire.src_pin_uuid.unwrap();

        let wire_src_position = q_src_pins
            .iter()
            .find(|(_, p)| p.uuid.eq(&wire_src_pin_uuid))
            .unwrap_or_else(|| panic!("No pin found with uuid: {}", wire_src_pin_uuid))
            .0
            .translation()
            .truncate();

        let wire_dest_position = if let Some(wire_dest_pin_uuid) = wire.dest_pin_uuid {
            q_dest_pins
                .iter()
                .find(|(_, p)| p.uuid.eq(&wire_dest_pin_uuid))
                .unwrap_or_else(|| panic!("No pin found with uuid: {}", wire_dest_pin_uuid))
                .0
                .translation()
                .truncate()
        } else {
            cursor_transform.translation.truncate()
        };

        let new_wire = shapes::Line(wire_src_position, wire_dest_position);
        *wire_path = ShapePath::build_as(&new_wire);
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

    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    if cursor.state == CursorState::Idle {
        for (bbox, pin_view) in q_wire_src_pins.iter() {
            if !bbox.point_in_bbox(cursor_transform.translation.truncate()) {
                continue;
            }

            // cursor is on pin
            let wire = commands
                .spawn(WireBundle::new(Wire {
                    src_pin_uuid: Some(pin_view.uuid),
                    dest_pin_uuid: None,
                    wire_points: Vec::new(),
                }))
                .id();
            cursor.state = CursorState::DraggingWire(wire);
            return;
        }
    } else if let CursorState::DraggingWire(wire_entity) = cursor.state {
        let Ok(mut wire) = q_wires.get_mut(wire_entity) else {
            return;
        };

        for (bbox, pin_view) in q_wire_dest_pins.iter() {
            if bbox.point_in_bbox(cursor_transform.translation.truncate()) {
                // connect wire to pin
                wire.dest_pin_uuid = Some(pin_view.uuid);
                cursor.state = CursorState::Idle;
                return;
            }
        }
    }
}

pub fn cancel_drag(
    input: Res<ButtonInput<KeyCode>>,
    mut q_cursor: Query<&mut Cursor>,
    mut commands: Commands,
) {
    let mut cursor = get_cursor_mut!(q_cursor);

    if !input.just_pressed(KeyCode::Escape) {
        return;
    }

    if let CursorState::DraggingWire(wire_entity) = cursor.state {
        cursor.state = CursorState::Idle;
        commands.entity(wire_entity).despawn_recursive();
    }
}

//TODO: make faster by not updating colors that havent changed.
/**
 * Updates all colors that are bound to a signal, e.g. pins or wires.
 */
#[allow(clippy::type_complexity)]
pub fn update_wire_signal_colors(
    q_wires: Query<(&Viewable<Wire>, &SignalState)>,
    mut q_wire_views: Query<&mut Stroke, With<WireView>>,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    // Color Wires
    for (wire_viewable, signal_state) in q_wires.iter() {
        let mut wire_stroke = q_wire_views.get_mut(wire_viewable.view().entity()).unwrap();

        let signal_wire_stroke = Stroke::new(
            match signal_state {
                SignalState::Low => render_settings.signal_low_color,
                SignalState::High => render_settings.signal_high_color,
            },
            render_settings.wire_line_width,
        );

        *wire_stroke = signal_wire_stroke;
    }
}

//TODO: rename maybe (e.g. WireCorner)
#[derive(Component)]
pub struct WirePoint;

#[derive(Bundle)]
pub struct WirePointBundle {
    wire_point: WirePoint,
    spatial_bundle: SpatialBundle,
}

impl WirePointBundle {
    pub fn new(position: Position) -> Self {
        Self {
            wire_point: WirePoint,
            spatial_bundle: SpatialBundle {
                transform: Transform::from_translation(position.to_translation(0.0)),
                ..default()
            },
        }
    }
}

pub fn create_wire_point(
    input: Res<ButtonInput<MouseButton>>,
    q_cursor: Query<(&Cursor, &Transform)>,
    mut q_wires: Query<&mut Wire>,
    mut commands: Commands,
) {
    let (cursor, cursor_transform) = get_cursor!(q_cursor);

    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    if let CursorState::DraggingWire(wire_entity) = cursor.state {
        if let Ok(mut wire) = q_wires.get_mut(wire_entity) {
            let position = Position::from_translation(cursor_transform.translation);
            let wire_point_entity = commands.spawn(WirePointBundle::new(position)).id();
            wire.wire_points.push(wire_point_entity);
        }
    }
}
