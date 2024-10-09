use bevy::prelude::*;

#[derive(Resource, Default, Reflect)]
pub struct DebugModeSettings {
    pub draw_bounding_boxes: bool,
    pub draw_entity_ids: bool,
}
