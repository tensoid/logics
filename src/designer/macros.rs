#[macro_export]
macro_rules! get_cursor {
    ($query:expr) => {
        $query.get_single().expect("No cursor entity in the scene.")
    };
}

#[macro_export]
macro_rules! get_cursor_mut {
    ($query:expr) => {
        $query
            .get_single_mut()
            .expect("No cursor entity in the scene.")
    };
}

#[macro_export]
macro_rules! find_descendant {
    ($q_children:expr, $view_entity:expr, $q_target:expr, $action:block) => {
        for child_entity in $q_children.iter_descendants($view_entity) {
            if let Ok(mut target) = $q_target.get_mut(child_entity) {
                $action
                break;
            }
        }
    };
}

#[macro_export]
macro_rules! find_descendants {
    ($q_children:expr, $view_entity:expr, $q_target:expr, $action:block) => {
        for child_entity in $q_children.iter_descendants($view_entity) {
            if let Ok(mut target) = $q_target.get_mut(child_entity) {
                $action
            }
        }
    };
}

#[macro_export]
macro_rules! get_model {
    ($q_parents:expr, $q_board_entities:expr, $q_models:expr, $wire_src_entity:expr) => {{
        let src_board_entity = $q_parents
            .iter_ancestors($wire_src_entity)
            .last()
            .expect("Entity does not have any Parents.");
        let src_model_entity = $q_board_entities
            .get(src_board_entity)
            .expect("Entity does not have a BoardEntityView Parent.")
            .viewable()
            .entity();

        $q_models.get(src_model_entity).ok()
    }};
}

#[macro_export]
macro_rules! get_model_mut {
    ($q_parents:expr, $q_board_entities:expr, $q_models:expr, $wire_src_entity:expr) => {{
        let src_board_entity = $q_parents
            .iter_ancestors($wire_src_entity)
            .last()
            .expect("Entity does not have any Parents.");
        let src_model_entity = $q_board_entities
            .get(src_board_entity)
            .expect("Entity does not have a BoardEntityView Parent.")
            .viewable()
            .entity();

        $q_models.get_mut(src_model_entity).ok()
    }};
}
