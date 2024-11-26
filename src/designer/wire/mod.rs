use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use moonshine_core::object::Object;
use moonshine_save::save::Save;
use moonshine_view::{BuildView, ViewCommands, Viewable};
use uuid::Uuid;

use crate::{
    get_cursor, get_cursor_mut, simulation::simulation::propagate_signals,
    ui::cursor_captured::IsCursorCaptured,
};

use super::{
    bounding_box::BoundingBox,
    cursor::{Cursor, CursorState},
    pin::PinView,
    position::Position,
    render_settings::CircuitBoardRenderingSettings,
    signal_state::SignalState,
};

//TODO: reimplement simulation
//TODO: reimplement copy paste
//TODO: implement wire delete (wire bbox)
//TODO: fix line jank (LineList)
//TODO: split into files
pub struct WirePlugin;

impl Plugin for WirePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (finish_wire_placement, create_wire_joint, create_wire)
                .chain()
                .run_if(resource_equals(IsCursorCaptured(false))),
        )
        .add_systems(
            Update,
            (
                cancel_wire_placement,
                /*despawn_dangling_wires,*/ update_wires,
            )
                .chain(),
        )
        .add_systems(Update, update_wire_signal_colors.after(propagate_signals));
        //TODO: observers or Changed<> Filter
    }
}

//TODO: rename
#[derive(Clone, Reflect, Debug, PartialEq, Eq, Hash)]
pub enum WireNode {
    Pin(Uuid),
    Joint(Entity),
}

#[derive(Component, Reflect, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[reflect(Component, Default)]
pub struct Wire {
    pub nodes: Vec<WireNode>,
}

#[derive(Bundle)]
pub struct WireBundle {
    pub wire: Wire,
    pub save: Save,
    pub signal_state: SignalState,
}

impl WireBundle {
    pub fn new(wire_nodes: Vec<WireNode>) -> Self {
        Self {
            wire: Wire { nodes: wire_nodes },
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
        for joint in wire.nodes.iter() {
            match joint {
                WireNode::Joint(joint_entity) => {}
                WireNode::Pin(pin_uuid) => {}
            }
        }

        // let is_dangling = wire.src_pin_uuid.is_none() || wire.dest_pin_uuid.is_none();
        // let is_currently_dragged =
        //     matches!(cursor.state, CursorState::DraggingWire(w) if w == wire_entity);

        // if is_dangling && !is_currently_dragged {
        //     commands.entity(wire_entity).despawn_recursive();
        //     continue;
        // }

        // let src_pin_exists = wire
        //     .src_pin_uuid
        //     .map(|uuid| q_pins.iter().any(|pin| pin.uuid == uuid))
        //     .unwrap_or(false);

        // let dest_pin_exists = wire
        //     .dest_pin_uuid
        //     .map(|uuid| q_pins.iter().any(|pin| pin.uuid == uuid))
        //     .unwrap_or(false);

        // if !src_pin_exists || !dest_pin_exists {
        //     commands.entity(wire_entity).despawn_recursive();
        // }
    }
}

#[allow(clippy::type_complexity)]
pub fn update_wires(
    q_wires: Query<(&mut Wire, &Viewable<Wire>, Entity)>,
    q_pins: Query<(&GlobalTransform, &PinView)>,
    q_wire_joints: Query<&Transform, With<WireJoint>>,
    q_cursor: Query<(&Cursor, &Transform)>,
    mut q_wire_views: Query<&mut Path, With<WireView>>,
) {
    let (cursor, cursor_transform) = get_cursor!(q_cursor);

    for (wire, wire_viewable, wire_entity) in q_wires.iter() {
        let mut wire_path = q_wire_views.get_mut(wire_viewable.view().entity()).unwrap();

        let mut points: Vec<Vec2> = wire
            .nodes
            .iter()
            .map(|wire_node| match wire_node {
                WireNode::Joint(joint) => q_wire_joints.get(*joint).unwrap().translation.truncate(),
                WireNode::Pin(pin_uuid) => q_pins
                    .iter()
                    .find(|(_, pin_view)| pin_view.uuid == *pin_uuid)
                    .unwrap()
                    .0
                    .translation()
                    .truncate(),
            })
            .collect();

        //TODO: optimize by checking if dragged first otherwise only change if changed,
        //maybe get dragged wire first then update changed ones

        if matches!(cursor.state, CursorState::DraggingWire(w) if w == wire_entity) {
            points.push(cursor_transform.translation.truncate());
        }

        let new_wire = shapes::Polygon {
            points,
            closed: false,
        };

        *wire_path = ShapePath::build_as(&new_wire);
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn create_wire(
    input: Res<ButtonInput<MouseButton>>,
    q_pins: Query<(&BoundingBox, &PinView)>,
    mut q_cursor: Query<(&mut Cursor, &Transform), With<Cursor>>,
    mut commands: Commands,
) {
    let (mut cursor, cursor_transform) = get_cursor_mut!(q_cursor);

    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    if cursor.state != CursorState::Idle {
        return;
    }

    for (bbox, pin_view) in q_pins.iter() {
        if !bbox.point_in_bbox(cursor_transform.translation.truncate()) {
            continue;
        }

        // cursor is on pin
        let wire = commands
            .spawn(WireBundle::new(vec![WireNode::Pin(pin_view.uuid)]))
            .id();

        cursor.state = CursorState::DraggingWire(wire);
        return;
    }
}

pub fn cancel_wire_placement(
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

#[derive(Component)]
pub struct WireJoint;

#[derive(Bundle)]
pub struct WireJointBundle {
    wire_joint: WireJoint,
    signal_state: SignalState,
    spatial_bundle: SpatialBundle,
}

impl WireJointBundle {
    pub fn new(position: Position) -> Self {
        Self {
            wire_joint: WireJoint,
            signal_state: SignalState::Low,
            spatial_bundle: SpatialBundle {
                transform: Transform::from_translation(position.to_translation(0.0)),
                ..default()
            },
        }
    }
}

pub fn create_wire_joint(
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
            let wire_joint_entity = commands.spawn(WireJointBundle::new(position)).id();
            wire.nodes.push(WireNode::Joint(wire_joint_entity));
        }
    }
}

pub fn finish_wire_placement(
    input: Res<ButtonInput<MouseButton>>,
    q_pins: Query<(&BoundingBox, &PinView)>,
    mut q_cursor: Query<(&mut Cursor, &Transform), With<Cursor>>,
    mut q_wires: Query<&mut Wire>,
) {
    let (mut cursor, cursor_transform) = get_cursor_mut!(q_cursor);

    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    let CursorState::DraggingWire(wire_entity) = cursor.state else {
        return;
    };

    let mut wire = q_wires.get_mut(wire_entity).unwrap();

    for (bbox, pin_view) in q_pins.iter() {
        if bbox.point_in_bbox(cursor_transform.translation.truncate()) {
            wire.nodes.push(WireNode::Pin(pin_view.uuid));
            cursor.state = CursorState::Idle;
            return;
        }
    }
}
