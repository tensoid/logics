//TODO: comment macros and maybe rename the get_model

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

//TODO: non mut version?
#[macro_export]
macro_rules! find_descendant {
    ($q_children:expr, $view_entity:expr, $q_target:expr, $closure:expr) => {
        for child_entity in $q_children.iter_descendants($view_entity) {
            if let Ok(mut target) = $q_target.get_mut(child_entity) {
                $closure(&mut target);
                break;
            }
        }
    };
}

// #[macro_export]
// macro_rules! find_descendants {
//     ($q_children:expr, $view_entity:expr, $q_target:expr, $closure:expr) => {
//         for child_entity in $q_children.iter_descendants($view_entity) {
//             if let Ok(mut target) = $q_target.get_mut(child_entity) {
//                 $closure(&mut target);
//             }
//         }
//     };
// }

//TODO: handle deletion in a stage where there can never be a view without its model in the update stage
#[macro_export]
macro_rules! get_model {
    ($q_parents:expr, $q_board_entities:expr, $q_models:expr, $wire_src_entity:expr) => {{
        let src_device = $q_parents
            .iter_ancestors($wire_src_entity)
            .last()
            .expect("Entity does not have any Parents.");
        let src_model_entity = $q_board_entities
            .get(src_device)
            .expect("Entity does not have a BoardEntityView Parent.")
            .viewable()
            .entity();

        $q_models.get(src_model_entity).ok()
    }};
}

#[macro_export]
macro_rules! get_model_mut {
    ($q_parents:expr, $q_board_entities:expr, $q_models:expr, $wire_src_entity:expr) => {{
        let src_device = $q_parents
            .iter_ancestors($wire_src_entity)
            .last()
            .expect("Entity does not have any Parents.");
        let src_model_entity = $q_board_entities
            .get(src_device)
            .expect("Entity does not have a BoardEntityView Parent.")
            .viewable()
            .entity();

        $q_models.get_mut(src_model_entity).ok()
    }};
}

//TODO: yeah this is bonkers. query trait maybe.
#[macro_export]
macro_rules! find_model_by_uuid {
    ($query:expr, $uuid:expr) => {
        $query.iter().find(|item| {
            // assuming the first component of the query is always ModelId
            item.0 .0 == $uuid
        })
    };
}

#[macro_export]
macro_rules! find_model_by_uuid_mut {
    ($query:expr, $uuid:expr) => {
        $query.iter_mut().find(|item| {
            // assuming the first component of the query is always ModelId
            item.0 .0 == $uuid
        })
    };
}
