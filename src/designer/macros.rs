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