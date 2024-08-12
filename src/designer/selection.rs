use bevy::{color::palettes::css::RED, math::bounding::Aabb2d, prelude::*};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::{Path, ShapeBundle},
    geometry::GeometryBuilder,
    shapes::{self, RectangleOrigin},
};
use moonshine_view::{View, Viewable};

use crate::{
    events::events::DeleteSelectedEvent, get_cursor, get_cursor_mut,
    ui::cursor_captured::IsCursorCaptured,
};

use super::{
    board_entity::{BoardEntityView, BoardEntityViewKind},
    bounding_box::{BoundingBox, BoundingShape},
    cursor::{Cursor, CursorState},
    render_settings::CircuitBoardRenderingSettings,
};

#[derive(Component)]
pub struct Selected;

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
                RED, //render_settings.board_entity_stroke_color_selected,
                2.0, //render_settings.board_entity_stroke_width,
            ),
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: Vec2::new(100.0, 100.0), //binary_input_extents,
                    ..default()
                }),
                spatial: SpatialBundle {
                    //transform: Transform::from_xyz(ev.position.x, ev.position.y, 0.0),
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct SelectionBox;

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
    q_selected_entities: Query<
        (&View<BoardEntityViewKind>, &BoundingBox),
        (With<Selected>, With<BoardEntityView>),
    >,
    q_not_selected_entities: Query<
        (&View<BoardEntityViewKind>, &BoundingBox),
        (Without<Selected>, With<BoardEntityView>),
    >,
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

    // update not selected entities
    for (view, bbox) in q_not_selected_entities.iter() {
        println!("Checking");
        if !bbox.intersects(selection_box_bbox) || !bbox.selectable {
            continue;
        }

        println!("Found selected");

        let model_entity = view.viewable().entity();
        commands.entity(model_entity).insert(Selected);
    }

    // update selected entities
    for (view, bbox) in q_selected_entities.iter() {
        if bbox.intersects(selection_box_bbox) || !bbox.selectable {
            continue;
        }

        //commands.entity(entity).remove::<Selected>();
    }

    // update selection box visual
    *path = GeometryBuilder::build_as(&shapes::Rectangle {
        extents: cursor_position - selection_box_position,
        origin: RectangleOrigin::BottomLeft,
    });
}

#[allow(clippy::type_complexity)]
pub fn drag_selected(
    mut q_cursor: Query<(&mut Cursor, Entity, &Transform)>,
    input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut q_selected_entities: Query<
        (Entity, &BoundingBox, &mut Transform),
        (With<Selected>, Without<Cursor>),
    >,
) {
    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    let (mut cursor, cursor_entity, cursor_transform) = get_cursor_mut!(q_cursor);
    let cursor_position = cursor_transform.translation.truncate();

    if cursor.state != CursorState::Idle {
        return;
    }

    if !q_selected_entities
        .iter()
        .any(|(_, bbox, _)| bbox.selectable && bbox.point_in_bbox(cursor_position))
    {
        return;
    }

    for (entity, _, mut transform) in q_selected_entities.iter_mut() {
        commands.entity(cursor_entity).add_child(entity);
        let position_diff = transform.translation - cursor_transform.translation;
        transform.translation = position_diff;
    }

    cursor.state = CursorState::Dragging;
}

#[allow(clippy::type_complexity)]
pub fn clear_selection(
    q_cursor: Query<(&Cursor, &Transform)>,
    input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    q_selected_entities: Query<(Entity, &BoundingBox), (With<Selected>, Without<Cursor>)>,
) {
    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    let (cursor, cursor_transform) = get_cursor!(q_cursor);
    let cursor_position = cursor_transform.translation.truncate();

    if cursor.state != CursorState::Idle {
        return;
    }

    if q_selected_entities
        .iter()
        .any(|(_, bbox)| bbox.selectable && bbox.point_in_bbox(cursor_position))
    {
        return;
    }

    for (entity, _) in q_selected_entities.iter() {
        commands.entity(entity).remove::<Selected>();
    }
}

#[allow(clippy::type_complexity)]
pub fn select_single(
    q_cursor: Query<(&Cursor, &Transform)>,
    input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    q_not_selected_entities: Query<(Entity, &BoundingBox), (Without<Selected>, Without<Cursor>)>,
) {
    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    let (cursor, cursor_transform) = get_cursor!(q_cursor);
    let cursor_position = cursor_transform.translation.truncate();

    if cursor.state != CursorState::Idle {
        return;
    }

    for (entity, bbox) in q_not_selected_entities.iter() {
        if bbox.selectable && bbox.point_in_bbox(cursor_position) {
            commands.entity(entity).insert(Selected);
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn stop_dragging(
    mut q_cursor: Query<(Entity, &mut Cursor, &Transform, Option<&Children>)>,
    input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut q_dragged_entities: Query<
        (Entity, &BoundingBox, &mut Transform),
        (With<Selected>, Without<Cursor>),
    >,
    cursor_captured: Res<IsCursorCaptured>,
) {
    let (cursor_entity, mut cursor, cursor_transform, cursor_children) = get_cursor_mut!(q_cursor);

    if cursor.state == CursorState::Dragging && input.just_released(MouseButton::Left) {
        cursor.state = CursorState::Idle;

        if cursor_captured.0 {
            commands.entity(cursor_entity).despawn_descendants();
            return;
        }

        for &cursor_child_entity in cursor_children.iter().flat_map(|c| c.iter()) {
            let (_, _, mut child_transform) =
                q_dragged_entities.get_mut(cursor_child_entity).unwrap();
            child_transform.translation =
                cursor_transform.translation + child_transform.translation;
        }

        commands.entity(cursor_entity).clear_children();
    }
}

pub fn delete_selected(
    mut commands: Commands,
    q_selected_entities: Query<Entity, With<Selected>>,
    mut delete_ev: EventReader<DeleteSelectedEvent>,
) {
    for _ in delete_ev.read() {
        for selected_entity in q_selected_entities.iter() {
            commands.entity(selected_entity).despawn_recursive();
        }
    }
}

//TODO: Find better way to do this, like with traits or commands or smth
pub fn highlight_selected(
    mut q_selected_entities: Query<&mut Stroke, With<Selected>>,
    mut q_not_selected_entities: Query<&mut Stroke, Without<Selected>>,
    render_settings: Res<CircuitBoardRenderingSettings>,
) {
    for mut stroke in q_selected_entities.iter_mut() {
        *stroke = Stroke::new(
            render_settings.board_entity_stroke_color_selected,
            render_settings.board_entity_stroke_width,
        );
    }

    for mut stroke in q_not_selected_entities.iter_mut() {
        *stroke = Stroke::new(
            render_settings.board_entity_stroke_color,
            render_settings.board_entity_stroke_width,
        );
    }
}
