use bevy::prelude::*;
use moonshine_view::View;

use crate::designer::devices::device::DeviceViewKind;

use super::{debug_mode_settings::DebugModeSettings, debug_mode_state::DebugModeState};

#[derive(Component)]
pub struct EntityIdDebugText;

/// Draws entity ids next to devices and wires.
/// Very crude implementation that needs to be built upon.
#[allow(clippy::type_complexity)]
pub fn draw_entity_ids(
    q_entities: Query<Entity, With<View<DeviceViewKind>>>,
    q_entity_id_debug_texts: Query<Entity, With<EntityIdDebugText>>,
    asset_server: Res<AssetServer>,
    debug_mode_settings: Res<DebugModeSettings>,
    debug_mode_state: Res<State<DebugModeState>>,
    mut commands: Commands,
) {
    let is_currently_active = !q_entity_id_debug_texts.is_empty();
    let should_be_active =
        debug_mode_state.eq(&DebugModeState::On) && debug_mode_settings.draw_entity_ids;

    if should_be_active && !is_currently_active {
        let font: Handle<Font> = asset_server.load("fonts/VCR_OSD_MONO.ttf");

        let text_style = TextStyle {
            font_size: 20.0, // TODO: settings
            color: Color::BLACK,
            font,
        };

        for entity in q_entities.iter() {
            commands.entity(entity).with_children(|cb| {
                cb.spawn((
                    Text2dBundle {
                        text: Text::from_section(entity.to_string(), text_style.clone())
                            .with_justify(JustifyText::Center),
                        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.1)),
                        ..default()
                    },
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
