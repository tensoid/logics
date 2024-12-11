use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Resource, PartialEq, Eq)]
pub struct IsCursorCaptured(pub bool);

// #[derive(Component)]
// pub struct NoCursorCapture;

pub fn check_cursor_captured(
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut is_cursor_captured: ResMut<IsCursorCaptured>,
    node_query: Query<
        (&ComputedNode, &GlobalTransform, &InheritedVisibility), /* , Without<NoCursorCapture>*/
    >,
) {
    let window = q_window.get_single().unwrap();
    is_cursor_captured.0 = window
        .cursor_position()
        .map(|cursor_position| {
            node_query
                .iter()
                .filter(|(_, _, visibility)| visibility.get())
                .any(|(&node, transform, ..)| {
                    let node_position = transform.translation().xy();
                    let half_size = 0.5 * node.size();
                    let min = node_position - half_size;
                    let max = node_position + half_size;
                    (min.x..max.x).contains(&cursor_position.x)
                        && (min.y..max.y).contains(&cursor_position.y)
                })
        })
        .unwrap_or(false);
}
