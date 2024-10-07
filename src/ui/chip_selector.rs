use bevy::prelude::*;

use crate::{
    designer::{devices::device::DeviceIds, position::Position},
    events::events::SpawnDeviceEvent,
};

use super::styles::*;

#[derive(Component)]
pub struct ChipSelector;

#[derive(Component)]
pub struct ChipButton;

pub fn spawn_chip_selector(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_device_ids: Res<DeviceIds>,
) {
    commands
        .spawn((
            ChipSelector,
            NodeBundle {
                style: chip_selector_style(),
                background_color: chip_selector_background_color(),
                border_color: chip_selector_border_color(),
                ..default()
            },
        ))
        .with_children(|cs| {
            for device_id in q_device_ids.devices.iter() {
                cs.spawn((
                    ChipButton,
                    ButtonBundle {
                        style: chip_button_style(),
                        background_color: chip_button_background_color(),
                        ..default()
                    },
                ))
                .with_children(|b| {
                    b.spawn(TextBundle::from_section(
                        device_id,
                        chip_button_text_style(&asset_server),
                    ));
                });
            }
        });
}

#[allow(clippy::type_complexity)]
pub fn chip_selector_button_interact(
    mut q_buttons: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<ChipButton>),
    >,
    q_texts: Query<&Text>,
    mut spawn_ev_writer: EventWriter<SpawnDeviceEvent>,
) {
    for (interaction, mut background_color, children) in q_buttons.iter_mut() {
        let button_text = q_texts.get(*children.first().unwrap()).unwrap();

        match *interaction {
            Interaction::None => {
                *background_color = chip_button_background_color();
            }
            Interaction::Hovered => {
                *background_color = chip_button_background_color_hovered();
            }
            Interaction::Pressed => {
                *background_color = chip_button_background_color_pressed();

                let device_id = button_text.sections.first().unwrap().value.clone();

                spawn_ev_writer.send(SpawnDeviceEvent {
                    device_id,
                    position: Position::ZERO,
                    init_drag: true,
                });
            }
        }
    }
}
