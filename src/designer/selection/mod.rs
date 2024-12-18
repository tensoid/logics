use bevy::{
    math::bounding::{Aabb2d, BoundingVolume},
    prelude::*,
};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::{Path, ShapeBundle},
    geometry::GeometryBuilder,
    shapes::{self, RectangleOrigin},
};
use moonshine_view::{View, Viewable};

use crate::{
    events::events::{DeleteEvent, SelectAllEvent},
    get_cursor, get_cursor_mut,
    ui::cursor_captured::IsCursorCaptured,
};

use super::{
    bounding_box::{BoundingBox, BoundingShape},
    cursor::{Cursor, CursorState},
    devices::device::{DeviceModel, DeviceView, DeviceViewKind},
    position::Position,
    render_settings::CircuitBoardRenderingSettings,
    wire::drag_wire,
};

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, select_all.run_if(on_event::<SelectAllEvent>()))
            .add_systems(Update, release_drag)
            .add_systems(Update, update_selection_box)
            .add_systems(
                Update,
                (
                    spawn_selection_box,
                    (select_single, start_drag).chain().after(drag_wire),
                    delete_selected, //TODO: remove cursor captured condition for delete
                )
                    .after(drag_wire)
                    .run_if(resource_equals(IsCursorCaptured(false))),
            )
            .add_systems(PostUpdate, update_dragged_entities_position)
            .add_systems(PostUpdate, highlight_selected); //TODO: observers?
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
pub struct SelectionOutline;

#[derive(Bundle)]
pub struct SelectionOutlineBundle {
    selection_outline: SelectionOutline,
    stroke: Stroke,
    shape_bundle: ShapeBundle,
}

impl SelectionOutlineBundle {
    pub fn new(render_settings: &CircuitBoardRenderingSettings, extents: Vec2) -> Self {
        Self {
            selection_outline: SelectionOutline,
            stroke: Stroke::new(
                render_settings.device_stroke_color_selected,
                render_settings.device_stroke_width,
            ),
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents,
                    ..default()
                }),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn select_all(mut commands: Commands, q_devices: Query<Entity, With<DeviceModel>>) {
    for device in q_devices.iter() {
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
            }),
            spatial: SpatialBundle {
                transform: Transform::from_xyz(cursor_position.x, cursor_position.y, 2.0),
                ..default()
            },
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

pub fn update_selection_box(
    mut q_selection_box: Query<(&mut Path, &Transform, Entity), With<SelectionBox>>,
    mut q_cursor: Query<(&mut Cursor, &Transform)>,
    input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    q_device_views: Query<(&View<DeviceViewKind>, &BoundingBox), With<DeviceView>>,
    q_selected: Query<(), With<Selected>>,
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

    // update selection box visual
    *path = GeometryBuilder::build_as(&shapes::Rectangle {
        extents: cursor_position - selection_box_position,
        origin: RectangleOrigin::BottomLeft,
    });
}

#[allow(clippy::type_complexity)]
pub fn select_single(
    q_cursor: Query<(&Cursor, &Transform)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    q_device_views: Query<(&View<DeviceViewKind>, &BoundingBox), With<DeviceView>>,
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

    let hovered_device = q_device_views
        .iter()
        .find(|(_, bbox)| bbox.selectable && bbox.point_in_bbox(cursor_position));

    if hovered_device.is_none() {
        return;
    }

    let hovered_device_model_entity = hovered_device.unwrap().0.viewable().entity();
    let is_hovered_device_selected = q_selected.get(hovered_device_model_entity).is_ok();
    let ctrl_clicked = key_input.pressed(KeyCode::ControlLeft);

    // toggle selection with ctrl left-click
    if ctrl_clicked {
        if is_hovered_device_selected {
            commands
                .entity(hovered_device_model_entity)
                .remove::<Selected>();
        } else {
            commands
                .entity(hovered_device_model_entity)
                .insert(Selected);
        }

        return;
    }

    // normal left-click
    if !is_hovered_device_selected {
        q_selected.iter().for_each(|(e, _)| {
            commands.entity(e).remove::<Selected>();
        });

        commands
            .entity(hovered_device_model_entity)
            .insert(Selected);
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
        let cursor_offset = position.xy() - cursor_transform.translation.truncate();
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
    q_dragged_board_entities: Query<(Entity, &Position, &Dragged), With<DeviceModel>>,
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
    mut q_dragged_board_entities: Query<(Entity, &mut Position, &Dragged), With<DeviceModel>>,
) {
    let cursor_transform = get_cursor_mut!(q_cursor);

    // update positions
    for (_, mut position, dragged) in q_dragged_board_entities.iter_mut() {
        *position = Position(cursor_transform.translation.truncate() + dragged.cursor_offset.xy());
    }
}

pub fn delete_selected(
    mut commands: Commands,
    q_selected_entities: Query<Entity, With<Selected>>,
    mut delete_ev: EventReader<DeleteEvent>,
) {
    for _ in delete_ev.read() {
        for selected_entity in q_selected_entities.iter() {
            commands.entity(selected_entity).despawn_recursive();
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn highlight_selected(
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
    q_selection_outlines: Query<(Entity, &Parent), With<SelectionOutline>>,
    mut commands: Commands,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    for viewable in q_selected_entities.iter() {
        let view_entity = viewable.view().entity();
        let bbox = q_bounding_boxes.get(view_entity).unwrap();
        commands.entity(view_entity).with_children(|cb| {
            let extents = match bbox.bounding_shape {
                BoundingShape::Aabb(aabb) => aabb.half_size() * Vec2::splat(2.0),
                BoundingShape::Circle(_) => panic!("tried to highlight non aabb bounding box"),
            };

            cb.spawn(SelectionOutlineBundle::new(&render_settings, extents));
        });
    }

    for deselected_entity in q_deselected.read() {
        if let Ok(viewable) = q_entities.get(deselected_entity) {
            let view_entity = viewable.view().entity();
            let selection_outline_entity = q_selection_outlines
                .iter()
                .find(|so| so.1.get() == view_entity)
                .unwrap() //TODO: crashed
                .0;

            commands.entity(selection_outline_entity).remove_parent();
            commands.entity(selection_outline_entity).despawn();
        }
    }
}
