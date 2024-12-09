use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};
use bevy_prototype_lyon::prelude::*;
use moonshine_core::{kind::Kind, object::Object};
use moonshine_view::{BuildView, RegisterView, ViewCommands, Viewable};
use uuid::Uuid;
use wire_joint::{create_wire_joint, WireJointModel};

pub mod wire_joint;

use crate::{
    find_model_by_uuid, get_cursor, get_cursor_mut, simulation::simulation::propagate_signals,
    ui::cursor_captured::IsCursorCaptured,
};

use super::{
    bounding_box::{BoundingBox, BoundingShape},
    cursor::{Cursor, CursorState},
    model::{Model, ModelId, ModelRegistry},
    pin::PinView,
    position::Position,
    render_settings::CircuitBoardRenderingSettings,
    signal::{Signal, SignalState},
};

//TODO: delete wire when pin is deleted

//TODO: implement wire delete_selected / select_single / highlighting (wire bbox)
//TODO: reimplement copy paste (single clipboard / same paste pipeline)
//TODO: Only ever access model, view only accessed from model itself for syncing
//TODO: fix line jank (LineList)
//TODO: quadradic/cubic curves wires
//TODO: split into files
pub struct WirePlugin;

impl Plugin for WirePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WireNodes>()
            .register_type::<WireModel>();

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

#[derive(Reflect, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[reflect(Component, Default)]
pub struct WireNodes(pub Vec<WireNode>);

impl Component for WireNodes {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_remove(|mut world, entity, component_id| {
            let model_registry = world.get_resource::<ModelRegistry>().unwrap();
            let wire_nodes = world.get::<WireNodes>(entity).unwrap();

            let mut entities_to_delete: Vec<Entity> = Vec::new();

            for wire_node in wire_nodes.0.iter() {
                let WireNode::Joint(wire_joint) = wire_node else {
                    continue;
                };

                entities_to_delete.push(model_registry.get_model_entity(wire_joint));
            }

            let mut commands = world.commands();
            for entity in entities_to_delete {
                commands.entity(entity).despawn_recursive();
            }
        });
    }
}

/// Marker component for wire models
#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct WireModel;

//TODO: go similar with other models? marker type as "XXXModel" and singular components like "wirenodes" instead of just "Wire"
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
            bounding_box: BoundingBox::wire_new(Vec::new(), render_settings.wire_line_width, true),
        }
    }
}

impl BuildView for WireModel {
    fn build(world: &World, object: Object<WireModel>, view: &mut ViewCommands<WireModel>) {
        let render_settings = world.resource::<CircuitBoardRenderingSettings>();
        view.insert(WireViewBundle::new(render_settings));
    }
}

// impl BuildView for WireModel {
//     fn build(world: &World, _: Object<WireModel>, view: &mut ViewCommands<WireModel>) {
//         let render_settings = world.resource::<CircuitBoardRenderingSettings>();

//         view.insert(WireViewBundle::new(render_settings));
//     }
// }

#[allow(clippy::type_complexity)]
pub fn update_wire_views(
    q_wires: Query<(&mut WireNodes, &Viewable<WireModel>, Entity)>,
    q_pins: Query<(&GlobalTransform, &PinView)>,
    q_wire_joints: Query<(&ModelId, &Position), With<WireJointModel>>,
    q_cursor: Query<(&Cursor, &Transform)>,
    mut q_wire_views: Query<&mut Path, With<WireView>>,
) {
    let (cursor, cursor_transform) = get_cursor!(q_cursor);

    for (wire, wire_viewable, wire_entity) in q_wires.iter() {
        let mut wire_path = q_wire_views.get_mut(wire_viewable.view().entity()).unwrap();

        //TODO: duplicate code, only update bounding box then use values from there
        let mut points: Vec<Vec2> = wire
            .0
            .iter()
            .map(|wire_node| match wire_node {
                WireNode::Joint(joint_uuid) => {
                    find_model_by_uuid!(q_wire_joints, *joint_uuid)
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

//TODO: performance
pub fn update_wire_bbox(
    q_wires: Query<(&WireNodes, &Viewable<WireModel>)>,
    mut q_wire_views: Query<&mut BoundingBox, With<WireView>>,
    q_wire_joints: Query<(&ModelId, &Position), With<WireJointModel>>,
    q_pins: Query<(&GlobalTransform, &PinView)>,
) {
    for (wire_nodes, wire_viewable) in q_wires.iter() {
        let wire_points: Vec<Vec2> = wire_nodes
            .0
            .iter()
            .map(|wire_node| match wire_node {
                WireNode::Joint(joint_uuid) => {
                    find_model_by_uuid!(q_wire_joints, *joint_uuid)
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

    let CursorState::DraggingWire(wire_entity) = cursor.state else {
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
