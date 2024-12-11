use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use moonshine_core::object::Object;
use moonshine_view::{BuildView, RegisterView, ViewCommands, Viewable};
use uuid::Uuid;
use wire_joint::{create_wire_joint, WireJointModel};

pub mod wire_joint;

use crate::{
    get_cursor, get_cursor_mut, simulation::simulation::propagate_signals,
    ui::cursor_captured::IsCursorCaptured,
};

use super::{
    bounding_box::{BoundingBox, BoundingShape},
    cursor::{Cursor, CursorState},
    model::{Model, ModelId, ModelRegistry},
    pin::PinView,
    position::Position,
    render_settings::CircuitBoardRenderingSettings,
    selection::Selected,
    signal::{Signal, SignalState},
};

//TODO: look into shape border radii
//TODO: look into font_smoothing
//TODO: implement shift to drag straight
//TODO: refactor highlight code (observers)
//TODO: Only ever access model, view only accessed from model itself for syncing
//TODO: fix line jank (LineList)
//TODO: split into files
pub struct WirePlugin;

impl Plugin for WirePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WireNodes>()
            .register_type::<WireJointModel>()
            .register_type::<WireModel>();

        app.add_observer(on_create_wire);
        app.add_observer(on_remove_wire);

        app.register_viewable::<WireModel>();
        app.add_systems(
            Update,
            (
                finish_wire_placement,
                create_wire_joint.before(propagate_signals),
                create_wire,
            )
                .chain()
                .run_if(resource_equals(IsCursorCaptured(false))),
        )
        .add_systems(Update, (cancel_wire_placement, update_wire_views).chain())
        .add_systems(
            Update,
            update_wire_view_signal_colors.after(propagate_signals),
        )
        .add_systems(Update, update_wire_drag_point.after(create_wire_joint))
        .add_systems(Update, update_wire_bbox.after(create_wire_joint));
        //TODO: observers or Changed<> Filter
    }
}

//TODO: rename
#[derive(Clone, Reflect, Debug, PartialEq, Eq, Hash)]
pub enum WireNode {
    Pin(Uuid),
    Joint(Uuid),
}

#[derive(Component, Reflect, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[reflect(Component, Default)]
pub struct WireNodes(pub Vec<WireNode>);

/// Marker component for wire models
#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct WireModel;

#[derive(Bundle)]
pub struct WireModelBundle {
    pub wire_model: WireModel,
    pub wire_nodes: WireNodes,
    pub model: Model,
    pub signal_state: SignalState,
}

impl WireModelBundle {
    pub fn new(wire_nodes: Vec<WireNode>) -> Self {
        Self {
            wire_model: WireModel,
            model: Model::new(),
            wire_nodes: WireNodes(wire_nodes),
            signal_state: SignalState::new(Signal::Low),
        }
    }
}

/// Creates observers when a wire is spawned
pub fn on_create_wire(trigger: Trigger<OnAdd, WireModel>, mut commands: Commands) {
    commands.entity(trigger.entity()).observe(on_select_wire);
    commands.entity(trigger.entity()).observe(on_deselect_wire);
}

/// Selects the wire joints when a wire is selected
fn on_select_wire(
    trigger: Trigger<OnAdd, Selected>,
    q_wires: Query<&WireNodes>,
    model_registry: Res<ModelRegistry>,
    mut commands: Commands,
) {
    let wire_nodes = q_wires.get(trigger.entity()).unwrap();

    for wire_node in wire_nodes.0.iter() {
        let WireNode::Joint(joint) = wire_node else {
            continue;
        };

        let joint_entity = model_registry.get_model_entity(joint);

        commands.entity(joint_entity).insert(Selected);
    }
}

/// Deselects the wire joints when a wire is deselected
fn on_deselect_wire(
    trigger: Trigger<OnRemove, Selected>,
    q_wires: Query<&WireNodes>,
    model_registry: Res<ModelRegistry>,
    mut commands: Commands,
) {
    let wire_nodes = q_wires.get(trigger.entity()).unwrap();

    for wire_node in wire_nodes.0.iter() {
        let WireNode::Joint(joint) = wire_node else {
            continue;
        };

        let joint_entity = model_registry.get_model_entity(joint);

        commands.entity(joint_entity).remove::<Selected>();
    }
}

/// Despawns the wire joints when a wire is removed
/// BUG: currently causes duplicate despawns of wire joints when the wire is selected during its deletion.
/// This is because when a wire is selected its joints are also selected causing it to despawn from the delete action as well as this observer.
/// Should not be a problem once wires can survive without needing to be connected to a device.
fn on_remove_wire(
    trigger: Trigger<OnRemove, WireNodes>,
    q_wires: Query<&WireNodes>,
    model_registry: Res<ModelRegistry>,
    mut commands: Commands,
) {
    let wire_nodes = q_wires.get(trigger.entity()).unwrap();

    let mut entities_to_delete: Vec<Entity> = Vec::new();

    for wire_node in wire_nodes.0.iter() {
        let WireNode::Joint(wire_joint) = wire_node else {
            continue;
        };

        entities_to_delete.push(model_registry.get_model_entity(wire_joint));
    }

    for entity in entities_to_delete {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
pub struct WireView;

#[derive(Bundle)]
pub struct WireViewBundle {
    wire_view: WireView,
    shape_bundle: ShapeBundle,
    stroke: Stroke,
    bounding_box: BoundingBox,
}

impl WireViewBundle {
    pub fn new(render_settings: &CircuitBoardRenderingSettings) -> Self {
        Self {
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Line(Vec2::ZERO, Vec2::ZERO)),
                transform: Transform::from_xyz(0.0, 0.0, 0.005),
                ..default()
            },
            stroke: Stroke::new(
                render_settings.signal_low_color,
                render_settings.wire_line_width,
            ),
            wire_view: WireView,
            bounding_box: BoundingBox::wire_new(Vec::new(), render_settings.wire_line_width, true),
        }
    }
}

impl BuildView for WireModel {
    fn build(world: &World, _: Object<WireModel>, mut view: ViewCommands<Self>) {
        let render_settings = world.resource::<CircuitBoardRenderingSettings>();
        view.insert(WireViewBundle::new(render_settings));
    }
}

#[allow(clippy::type_complexity)]
pub fn update_wire_views(
    q_wires: Query<(&mut WireNodes, &Viewable<WireModel>, Entity)>,
    q_pins: Query<(&GlobalTransform, &PinView)>,
    q_wire_joints: Query<(&ModelId, &Position), With<WireJointModel>>,
    model_registry: Res<ModelRegistry>,
    q_cursor: Query<&Cursor>,
    mut q_wire_views: Query<&mut Path, With<WireView>>,
) {
    let cursor = get_cursor!(q_cursor);

    for (wire, wire_viewable, wire_entity) in q_wires.iter() {
        let mut wire_path = q_wire_views.get_mut(wire_viewable.view().entity()).unwrap();

        //TODO: duplicate code, only update bounding box then use values from there
        let mut points: Vec<Vec2> = wire
            .0
            .iter()
            .map(|wire_node| match wire_node {
                WireNode::Joint(joint_uuid) => {
                    q_wire_joints
                        .get(model_registry.get_model_entity(joint_uuid))
                        .unwrap()
                        .1
                         .0
                }
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

        // straight wire when holding shift
        if let CursorState::DraggingWire(wire, drag_position) = cursor.state {
            if wire_entity == wire {
                points.push(drag_position);
            }
        }

        let new_wire = shapes::Polygon {
            points,
            closed: false,
        };

        *wire_path = ShapePath::build_as(&new_wire);
    }
}

//TODO: performance
pub fn update_wire_bbox(
    q_wires: Query<(&WireNodes, &Viewable<WireModel>)>,
    mut q_wire_views: Query<&mut BoundingBox, With<WireView>>,
    q_wire_joints: Query<(&ModelId, &Position), With<WireJointModel>>,
    q_pins: Query<(&GlobalTransform, &PinView)>,
    model_registry: Res<ModelRegistry>,
) {
    for (wire_nodes, wire_viewable) in q_wires.iter() {
        //TODO: duplicate
        let wire_points: Vec<Vec2> = wire_nodes
            .0
            .iter()
            .map(|wire_node| match wire_node {
                WireNode::Joint(joint_uuid) => {
                    q_wire_joints
                        .get(model_registry.get_model_entity(joint_uuid))
                        .unwrap()
                        .1
                         .0
                }
                WireNode::Pin(pin_uuid) => q_pins
                    .iter()
                    .find(|(_, pin_view)| pin_view.uuid == *pin_uuid)
                    .unwrap()
                    .0
                    .translation()
                    .truncate(),
            })
            .collect();

        let mut wire_bbox = q_wire_views.get_mut(wire_viewable.view().entity()).unwrap();

        match &mut wire_bbox.bounding_shape {
            BoundingShape::Wire(wire_shape) => {
                wire_shape.points = wire_points;
            }
            _ => panic!("invalid bounding shape for wire"),
        }
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
            .spawn(WireModelBundle::new(vec![WireNode::Pin(pin_view.uuid)]))
            .id();

        cursor.state = CursorState::DraggingWire(wire, cursor_transform.translation.truncate());
        return;
    }
}

///HACK: terrible, but only temporary until grid snapping is implemented
pub fn update_wire_drag_point(
    mut q_cursor: Query<(&mut Cursor, &Transform), With<Cursor>>,
    q_wires: Query<&mut WireNodes>,
    input: Res<ButtonInput<KeyCode>>,
    q_wire_joints: Query<(&ModelId, &Position), With<WireJointModel>>,
    model_registry: Res<ModelRegistry>,
    q_pins: Query<(&GlobalTransform, &PinView)>,
) {
    let (mut cursor, cursor_transform) = get_cursor_mut!(q_cursor);

    let CursorState::DraggingWire(wire, ref mut pos) = cursor.state else {
        return;
    };

    //straight wire when holding shift
    if !input.pressed(KeyCode::ShiftLeft) {
        *pos = cursor_transform.translation.truncate();
    } else {
        //HACK: duplicate code
        let last_point = match q_wires.get(wire).unwrap().0.last().unwrap() {
            WireNode::Joint(joint_uuid) => {
                q_wire_joints
                    .get(model_registry.get_model_entity(joint_uuid))
                    .unwrap()
                    .1
                     .0
            }
            WireNode::Pin(pin_uuid) => q_pins
                .iter()
                .find(|(_, pin_view)| pin_view.uuid == *pin_uuid)
                .unwrap()
                .0
                .translation()
                .truncate(),
        };

        let delta = (last_point - cursor_transform.translation.truncate()).abs();

        let mut new_point = cursor_transform.translation.truncate();
        if delta.x > delta.y {
            new_point.y = last_point.y;
        } else {
            new_point.x = last_point.x;
        }

        *pos = new_point;
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

    if let CursorState::DraggingWire(wire_entity, _) = cursor.state {
        cursor.state = CursorState::Idle;
        commands.entity(wire_entity).despawn_recursive();
    }
}

//TODO: make faster by not updating colors that havent changed.
/**
 * Updates all colors that are bound to a signal, e.g. pins or wires.
 */
#[allow(clippy::type_complexity)]
pub fn update_wire_view_signal_colors(
    q_wires: Query<(&Viewable<WireModel>, &SignalState)>,
    mut q_wire_views: Query<&mut Stroke, With<WireView>>,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    // Color Wires
    for (wire_viewable, signal_state) in q_wires.iter() {
        let mut wire_stroke = q_wire_views.get_mut(wire_viewable.view().entity()).unwrap();

        let signal_wire_stroke = Stroke::new(
            match signal_state.get_signal() {
                Signal::Low => render_settings.signal_low_color,
                Signal::High => render_settings.signal_high_color,
                Signal::Conflict => render_settings.signal_conflict_color,
            },
            render_settings.wire_line_width,
        );

        *wire_stroke = signal_wire_stroke;
    }
}

pub fn finish_wire_placement(
    input: Res<ButtonInput<MouseButton>>,
    q_pins: Query<(&BoundingBox, &PinView)>,
    mut q_cursor: Query<(&mut Cursor, &Transform), With<Cursor>>,
    mut q_wires: Query<&mut WireNodes>,
) {
    let (mut cursor, cursor_transform) = get_cursor_mut!(q_cursor);

    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    let CursorState::DraggingWire(wire_entity, _) = cursor.state else {
        return;
    };

    let mut wire = q_wires.get_mut(wire_entity).unwrap();

    for (bbox, pin_view) in q_pins.iter() {
        if bbox.point_in_bbox(cursor_transform.translation.truncate()) {
            wire.0.push(WireNode::Pin(pin_view.uuid));
            cursor.state = CursorState::Idle;
            return;
        }
    }
}
