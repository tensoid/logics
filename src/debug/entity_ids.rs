use bevy::prelude::*;
use moonshine_view::View;

use crate::{assets::common_assets::CommonAssets, designer::devices::device::DeviceViewKind};

use super::{debug_mode_settings::DebugModeSettings, debug_mode_state::DebugModeState};

#[derive(Component)]
pub struct EntityIdDebugText;

/// Draws entity ids next to devices and wires.
/// Very crude implementation that needs to be built upon.
#[allow(clippy::type_complexity)]
pub fn draw_entity_ids(
    q_entities: Query<Entity, With<View<DeviceViewKind>>>,
    q_entity_id_debug_texts: Query<Entity, With<EntityIdDebugText>>,
    common_assets: Res<CommonAssets>,
    debug_mode_settings: Res<DebugModeSettings>,
    debug_mode_state: Res<State<DebugModeState>>,
    mut commands: Commands,
) {
    let is_currently_active = !q_entity_id_debug_texts.is_empty();
    let should_be_active =
        debug_mode_state.eq(&DebugModeState::On) && debug_mode_settings.draw_entity_ids;

    if should_be_active && !is_currently_active {
        for entity in q_entities.iter() {
            commands.entity(entity).with_children(|cb| {
                cb.spawn((
                    Text2d::new(entity.to_string()),
                    TextLayout::new_with_justify(JustifyText::Center),
                    TextFont {
                        font: common_assets.font.clone(),
                        font_size: 20.0, // TODO: settings
                        ..default()
                    },
                    TextColor(Color::BLACK),
                    Transform::from_translation(Vec3::new(0.0, 10.0, 0.1)),
                    EntityIdDebugText,
                ));
            });
        }
    } else if !should_be_active && is_currently_active {
        for entity in q_entity_id_debug_texts.iter() {
            commands.entity(entity).remove_parent();
            commands.entity(entity).despawn_recursive();
        }
    }
}
