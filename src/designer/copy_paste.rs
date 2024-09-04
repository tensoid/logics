use bevy::{ecs::entity::EntityHashMap, prelude::*};

use super::{board_entity::BoardEntityModel, selection::Selected};

#[derive(Resource)]
pub struct Clipboard(DynamicScene);

pub fn copy(
    q_selected: Query<Entity, (With<Selected>, With<BoardEntityModel>)>,
    mut clipboard: ResMut<Clipboard>,
    world: &World,
) {
    let dynamic_scene = DynamicSceneBuilder::from_world(world)
        .extract_entities(q_selected.iter())
        .build();

    clipboard.0 = dynamic_scene;
}

pub fn paste(world: &mut World) {
    let clipboard = world
        .remove_resource::<Clipboard>()
        .expect("Clipboard Resource not initialized.");

    clipboard
        .0
        .write_to_world(world, &mut EntityHashMap::default());

    world.insert_resource(clipboard);
}
