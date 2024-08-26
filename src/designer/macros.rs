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
