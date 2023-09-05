use bevy::prelude::*;

#[derive(Component)]
pub struct Wire {
    pub src_pin: Option<Entity>,
    pub dest_pin: Option<Entity>,
}
