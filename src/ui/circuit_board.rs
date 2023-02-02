use crate::simulation::chip::{Chip, ChipExtents};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Resource, PartialEq)]
pub enum CursorState {
    Idle,
    DraggingChip(Entity),
}

pub fn spawn_chip_at_cursor(
    input: Res<Input<MouseButton>>,
    commands: Commands,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    if input.just_pressed(MouseButton::Right) {
        let window = windows.get_primary().unwrap();
        if let Some(cursor_screen_pos) = window.cursor_position() {
            let (camera, camera_transform) = q_camera.single();

            let cursor_world_pos: Vec2 =
                screen_to_world_space(window, camera, camera_transform, cursor_screen_pos);

            spawn_chip(commands, cursor_world_pos);
        }
    }
}

pub fn spawn_chip(mut commands: Commands, position: Vec2) {
    //TODO: take max of input and output pins and adjust size accordingly
    let chip_extents: Vec2 = Vec2::new(100.0, 60.0);
    let shape = shapes::Rectangle {
        extents: chip_extents,
        ..default()
    };

    commands
        .spawn(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode {
                options: FillOptions::default(),
                color: Color::YELLOW,
            }),
            Transform::from_xyz(position.x, position.y, 0.0),
        ))
        .insert(Chip)
        .insert(ChipExtents(chip_extents));
}

// pub fn update_cursor_state() {

// }

pub fn screen_to_world_space(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    position: Vec2,
) -> Vec2 {
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let ndc = (position / window_size) * 2.0 - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    ndc_to_world.project_point3(ndc.extend(-1.0)).truncate()
}

pub fn drag_chip(
    input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), Without<Chip>>,
    mut q_chips: Query<
        (&GlobalTransform, &mut Transform, &ChipExtents, Entity),
        (With<Chip>, Without<Camera>),
    >,
    mut cursor_state: ResMut<CursorState>,
) {
    let window = windows.get_primary().unwrap();
    let (camera, camera_transform) = q_camera.single();

    if let Some(cursor_screen_pos) = window.cursor_position() {
        let cursor_position: Vec2 =
            screen_to_world_space(window, camera, camera_transform, cursor_screen_pos);

        if input.just_pressed(MouseButton::Left) {
            for (chip_global_transform, _, chip_extents, chip_entity) in q_chips.iter_mut() {
                let chip_position: Vec2 = Vec2::new(
                    chip_global_transform.translation().x,
                    chip_global_transform.translation().y,
                );

                let cursor_on_chip: bool = cursor_position.x
                    >= chip_position.x - (chip_extents.0.x / 2.0)
                    && cursor_position.x <= chip_position.x + (chip_extents.0.x / 2.0)
                    && cursor_position.y >= chip_position.y - (chip_extents.0.y / 2.0)
                    && cursor_position.y <= chip_position.y + (chip_extents.0.y / 2.0);

                if !cursor_on_chip {
                    continue;
                }

                *cursor_state = CursorState::DraggingChip(chip_entity);
                return;
            }
        }

        if input.pressed(MouseButton::Left) {
            if let CursorState::DraggingChip(dragged_chip_entity) = *cursor_state {
                for (_, mut chip_transform, _, chip_entity) in q_chips.iter_mut() {
                    if chip_entity != dragged_chip_entity {
                        continue;
                    }

                    chip_transform.translation =
                        cursor_position.extend(chip_transform.translation.z);
                }

                return;
            }
        }

        if input.just_released(MouseButton::Left) {
            *cursor_state = CursorState::Idle;
        }
    }
}
