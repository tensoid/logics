use bevy::prelude::*;

#[derive(Component)]
pub struct Wire {
    pub dest_pin: Option<Entity>,
}
