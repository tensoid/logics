use bevy::{
    math::bounding::{Aabb2d, BoundingVolume},
    prelude::*,
};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::{Path, ShapeBundle},
    geometry::GeometryBuilder,
    path::ShapePath,
    shapes::{self, RectangleOrigin},
};
use moonshine_view::{View, Viewable};

use crate::{
    events::{DeleteEvent, SelectAllEvent},
    find_descendant, get_cursor, get_cursor_mut, get_model,
    ui::cursor_captured::IsCursorCaptured,
};

use super::{
    bounding_box::{BoundingBox, BoundingShape},
    cursor::{Cursor, CursorState},
    devices::device::{DeviceModel, DeviceView, DeviceViewKind},
    model::{ModelId, ModelRegistry},
    pin::PinView,
    position::Position,
    render_settings::CircuitBoardRenderingSettings,
    wire::{create_wire, wire_joint::WireJointModel, WireModel, WireNode, WireNodes, WireView},
};

//TODO: split file, highlighting, selecting(box)
pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, select_all.run_if(on_event::<SelectAllEvent>))
            .add_systems(Update, release_drag)
            .add_systems(Update, update_selection_box)
            .add_systems(
                Update,
                (spawn_selection_box, (select_single, start_drag).chain())
                    .after(create_wire)
                    .run_if(resource_equals(IsCursorCaptured(false))),
            )
            .add_systems(PostUpdate, delete_selected.run_if(on_event::<DeleteEvent>))
            .add_systems(PostUpdate, update_dragged_entities_position)
            .add_systems(PostUpdate, highlight_selected_wires)
            .add_systems(PostUpdate, highlight_selected_devices); //TODO: observers?
    }
}

#[derive(Component)]
pub struct Dragged {
    pub cursor_offset: Position,
}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct SelectionBox;

#[derive(Component)]
pub struct DeviceSelectionOutline;

#[derive(Bundle)]
pub struct DeviceSelectionOutlineBundle {
    selection_outline: DeviceSelectionOutline,
    stroke: Stroke,
    shape_bundle: ShapeBundle,
}

impl DeviceSelectionOutlineBundle {
    pub fn new(render_settings: &CircuitBoardRenderingSettings, extents: Vec2) -> Self {
        Self {
            selection_outline: DeviceSelectionOutline,
            stroke: Stroke::new(
                render_settings.device_stroke_color_selected,
                render_settings.device_stroke_width,
            ),
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct WireSelectionOutline;

#[derive(Bundle)]
pub struct WireSelectionOutlineBundle {
    selection_outline: WireSelectionOutline,
    stroke: Stroke,
    shape_bundle: ShapeBundle,
}

impl WireSelectionOutlineBundle {
    pub fn new(render_settings: &CircuitBoardRenderingSettings) -> Self {
        Self {
            selection_outline: WireSelectionOutline,
            stroke: Stroke::new(
                render_settings.device_stroke_color_selected,
                render_settings.wire_line_width + 2.0,
            ),
            shape_bundle: ShapeBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
                ..default()
            },
        }
    }
}

//TODO: fix after bounding box on model by checking for either selectable bounding box or selectable component
pub fn select_all(
    mut commands: Commands,
    q_entities: Query<Entity, Or<(With<DeviceModel>, With<WireModel>, With<WireJointModel>)>>,
) {
    for device in q_entities.iter() {
        commands.entity(device).insert(Selected);
    }
}

pub fn spawn_selection_box(
    input: Res<ButtonInput<MouseButton>>,
    mut q_cursor: Query<(&mut Cursor, &Transform)>,
    q_bboxes: Query<&BoundingBox>,
    mut commands: Commands,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    let (mut cursor, cursor_transform) = get_cursor_mut!(q_cursor);
    let cursor_position = cursor_transform.translation.truncate();
    if cursor.state != CursorState::Idle {
        return;
    }

    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    if q_bboxes
        .iter()
        .any(|bbox| bbox.selectable && bbox.point_in_bbox(cursor_position))
    {
        return;
    }

    //TODO: bundle
    commands.spawn((
        SelectionBox,
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                extents: Vec2::ZERO,
                origin: RectangleOrigin::BottomLeft,
                ..default()
            }),
            transform: Transform::from_xyz(cursor_position.x, cursor_position.y, 2.0),
            ..default()
        },
        Stroke::new(
            render_settings.selection_box_stroke_color,
            render_settings.selection_box_stroke_width,
        ),
        Fill::color(render_settings.selection_box_fill_color),
    ));

    cursor.state = CursorState::Selecting;
}

//TODO: split into smaller functions
pub fn update_selection_box(
    mut q_selection_box: Query<(&mut Path, &Transform, Entity), With<SelectionBox>>,
    mut q_cursor: Query<(&mut Cursor, &Transform)>,
    input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    q_device_views: Query<(&View<DeviceViewKind>, &BoundingBox), With<DeviceView>>,
    q_selected: Query<(), With<Selected>>,
    q_wires: Query<(&View<WireModel>, &BoundingBox), With<WireView>>,
) {
    let (mut cursor, cursor_transform) = get_cursor_mut!(q_cursor);
    let cursor_position = cursor_transform.translation.truncate();

    let Ok((mut path, selection_box_transform, selection_box_entity)) =
        q_selection_box.get_single_mut()
    else {
        return;
    };

    let selection_box_position = selection_box_transform.translation.truncate();

    // despawn selection box
    if input.just_released(MouseButton::Left) {
        commands.entity(selection_box_entity).despawn_recursive();
        cursor.state = CursorState::Idle;
        return;
    }

    let selection_box_bbox = &BoundingBox {
        bounding_shape: BoundingShape::Aabb(Aabb2d::new(
            Vec2::new(
                (cursor_position.x + selection_box_position.x) / 2.0,
                (cursor_position.y + selection_box_position.y) / 2.0,
            ),
            ((cursor_position - selection_box_position) / Vec2::splat(2.0)).abs(),
        )),
        selectable: false,
        offset: Vec2::ZERO,
    };

    // update selected entities
    for (view, bbox) in q_device_views.iter() {
        if !bbox.selectable {
            continue;
        }

        let model_entity = view.viewable().entity();
        let intersects = bbox.intersects(selection_box_bbox);
        let is_selected = q_selected.get(model_entity).is_ok();

        if intersects && !is_selected {
            commands.entity(model_entity).insert(Selected);
        } else if !intersects && is_selected {
            commands.entity(model_entity).remove::<Selected>();
        }
    }

    //TODO: fix this jank ^
    for (view, bbox) in q_wires.iter() {
        if !bbox.selectable {
            continue;
        }

        let model_entity = view.viewable().entity();
        let intersects = bbox.intersects(selection_box_bbox);
        let is_selected = q_selected.get(model_entity).is_ok();

        if intersects && !is_selected {
            commands.entity(model_entity).insert(Selected);
        } else if !intersects && is_selected {
            commands.entity(model_entity).remove::<Selected>();
        }
    }

    // update selection box visual
    *path = GeometryBuilder::build_as(&shapes::Rectangle {
        extents: cursor_position - selection_box_position,
        origin: RectangleOrigin::BottomLeft,
        ..default()
    });
}

#[allow(clippy::type_complexity)]
pub fn select_single(
    q_cursor: Query<(&Cursor, &Transform)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    q_device_views: Query<(&View<DeviceViewKind>, &BoundingBox)>,
    q_wire_views: Query<(&View<WireModel>, &BoundingBox)>,
    q_selected: Query<(Entity, &Position), (With<Selected>, Without<Dragged>)>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    let (cursor, cursor_transform) = get_cursor!(q_cursor);
    let cursor_position = cursor_transform.translation.truncate();

    if cursor.state != CursorState::Idle {
        return;
    }

    let hovered = q_device_views
        .iter()
        .map(|(view, bbox)| (view.viewable().entity(), bbox))
        .chain(
            q_wire_views
                .iter()
                .map(|(view, bbox)| (view.viewable().entity(), bbox)),
        )
        .find(|(_, bbox)| bbox.selectable && bbox.point_in_bbox(cursor_position))
        .map(|(e, _)| e);

    let Some(hovered_entity) = hovered else {
        return;
    };

    let is_hovered_device_selected = q_selected.get(hovered_entity).is_ok();
    let ctrl_clicked = key_input.pressed(KeyCode::ControlLeft);

    // toggle selection with ctrl left-click
    if ctrl_clicked {
        if is_hovered_device_selected {
            commands.entity(hovered_entity).remove::<Selected>();
        } else {
            commands.entity(hovered_entity).insert(Selected);
        }

        return;
    }

    // normal left-click
    if !is_hovered_device_selected {
        q_selected.iter().for_each(|(e, _)| {
            commands.entity(e).remove::<Selected>();
        });

        commands.entity(hovered_entity).insert(Selected);
    }
}

#[allow(clippy::type_complexity)]
pub fn start_drag(
    mut q_cursor: Query<(&mut Cursor, Entity, &Transform)>,
    input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    q_device_views: Query<(&View<DeviceViewKind>, &BoundingBox), With<DeviceView>>,
    q_selected: Query<(Entity, &Position), (With<Selected>, Without<Dragged>)>,
) {
    let (mut cursor, _, cursor_transform) = get_cursor_mut!(q_cursor);
    let cursor_position = cursor_transform.translation.truncate();

    // check if drag is started
    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    if cursor.state != CursorState::Idle {
        return;
    }

    if !q_device_views
        .iter()
        .any(|(_, bbox)| bbox.selectable && bbox.point_in_bbox(cursor_position))
    {
        return;
    }

    // add Dragged component to selected entities
    for (selected_entity, position) in q_selected.iter() {
        let cursor_offset = position.0 - cursor_transform.translation.truncate();
        commands.entity(selected_entity).insert(Dragged {
            cursor_offset: Position(cursor_offset),
        });
    }

    cursor.state = CursorState::Dragging;
}

pub fn release_drag(
    cursor_captured: Res<IsCursorCaptured>,
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    mut q_cursor: Query<(&mut Cursor, Entity, &Transform)>,
    q_dragged_board_entities: Query<(Entity, &Position, &Dragged)>,
) {
    let (mut cursor, _, _) = get_cursor_mut!(q_cursor);

    if !input.just_released(MouseButton::Left) || cursor.state != CursorState::Dragging {
        return;
    }

    if cursor_captured.0 {
        q_dragged_board_entities.iter().for_each(|(e, _, _)| {
            commands.entity(e).despawn_recursive();
        });
    } else {
        q_dragged_board_entities.iter().for_each(|(e, _, _)| {
            commands.entity(e).remove::<Dragged>();
        });
    }

    cursor.state = CursorState::Idle;
}

#[allow(clippy::type_complexity)]
pub fn update_dragged_entities_position(
    mut q_cursor: Query<&Transform, With<Cursor>>,
    mut q_dragged_board_entities: Query<(Entity, &mut Position, &Dragged)>,
) {
    let cursor_transform = get_cursor_mut!(q_cursor);

    // update positions
    for (_, mut position, dragged) in q_dragged_board_entities.iter_mut() {
        *position = Position(cursor_transform.translation.truncate() + dragged.cursor_offset.0);
    }
}

pub fn delete_selected(mut commands: Commands, q_selected_entities: Query<Entity, With<Selected>>) {
    for selected_entity in q_selected_entities.iter() {
        //HACK: try because of duplicate wire joints deletions
        commands.entity(selected_entity).try_despawn_recursive();
    }
}

#[allow(clippy::type_complexity)]
pub fn highlight_selected_devices(
    q_selected_entities: Query<
        &Viewable<DeviceViewKind>,
        (
            With<Selected>,
            Or<(Added<Selected>, Added<Viewable<DeviceViewKind>>)>,
        ),
    >,
    q_entities: Query<&Viewable<DeviceViewKind>>,
    mut q_deselected: RemovedComponents<Selected>,
    q_bounding_boxes: Query<&BoundingBox>,
    q_selection_outlines: Query<(Entity, &Parent), With<DeviceSelectionOutline>>,
    mut commands: Commands,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    for viewable in q_selected_entities.iter() {
        let view_entity = viewable.view().entity();
        let bbox = q_bounding_boxes.get(view_entity).unwrap();
        commands.entity(view_entity).with_children(|cb| {
            let extents = match bbox.bounding_shape {
                BoundingShape::Aabb(aabb) => aabb.half_size() * Vec2::splat(2.0),
                _ => panic!("invalid bounding shape on device"),
            };

            cb.spawn(DeviceSelectionOutlineBundle::new(&render_settings, extents));
        });
    }

    for deselected_entity in q_deselected.read() {
        if let Ok(viewable) = q_entities.get(deselected_entity) {
            let view_entity = viewable.view().entity();
            let selection_outline_entity = q_selection_outlines
                .iter()
                .find(|so| so.1.get() == view_entity)
                .unwrap()
                .0;

            commands.entity(selection_outline_entity).remove_parent();
            commands.entity(selection_outline_entity).despawn();
        }
    }
}

//TODO: performance, and really bad implementation
#[allow(clippy::too_many_arguments)]
pub fn highlight_selected_wires(
    q_selected_wires: Query<(&WireNodes, &Viewable<WireModel>), With<Selected>>,
    q_wires: Query<&Viewable<WireModel>>,
    mut q_deselected: RemovedComponents<Selected>,
    mut q_selection_outlines: Query<(Entity, &Parent, &mut Path), With<WireSelectionOutline>>,
    mut commands: Commands,
    render_settings: Res<CircuitBoardRenderingSettings>,
    q_pins: Query<(&GlobalTransform, &PinView)>,
    q_wire_joints: Query<(&ModelId, &Position), With<WireJointModel>>,
    q_parents: Query<&Parent>,
    q_wire_views: Query<&View<WireModel>>,
    q_children: Query<&Children>,
    model_registry: Res<ModelRegistry>,
) {
    // add outline if missing
    for (_, viewable) in q_selected_wires.iter() {
        let view_entity = viewable.view().entity();
        let mut has_outline = false;
        find_descendant!(q_children, view_entity, q_selection_outlines, |_| {
            has_outline = true;
        });

        if !has_outline {
            commands.entity(view_entity).with_children(|cb| {
                cb.spawn(WireSelectionOutlineBundle::new(&render_settings));
            });
        }
    }

    // remove outline for deselected wires
    for deselected_entity in q_deselected.read() {
        if let Ok(viewable) = q_wires.get(deselected_entity) {
            let view_entity = viewable.view().entity();
            let selection_outline_entity = q_selection_outlines
                .iter()
                .find(|so| so.1.get() == view_entity)
                .unwrap()
                .0;

            commands.entity(selection_outline_entity).remove_parent();
            commands.entity(selection_outline_entity).despawn();
        }
    }

    for (selection_outline_entity, _, mut selection_outline_path) in q_selection_outlines.iter_mut()
    {
        if let Some((wire_nodes, _)) = get_model!(
            q_parents,
            q_wire_views,
            q_selected_wires,
            selection_outline_entity
        ) {
            //TODO: duplicate code
            let points: Vec<Vec2> = wire_nodes
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

            *selection_outline_path = ShapePath::build_as(&shapes::Polygon {
                points,
                closed: false,
            });
        }
    }
}
