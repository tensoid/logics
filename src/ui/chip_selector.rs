use bevy::{color::palettes::css::GRAY, prelude::*, text::FontSmoothing};

use crate::{
    assets::common_assets::CommonAssets,
    designer::{devices::device::DeviceIds, position::Position},
    events::SpawnDeviceEvent,
};

#[derive(Component)]
pub struct ChipSelector;

#[derive(Component)]
pub struct ChipButton;

pub fn spawn_chip_selector(
    mut commands: Commands,
    common_assets: Res<CommonAssets>,
    q_device_ids: Res<DeviceIds>,
) {
    commands
        .spawn((
            ChipSelector,
            Node {
                width: Val::Vw(20.0),
                height: Val::Vh(100.0),
                justify_self: JustifySelf::End,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                border: UiRect::left(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::WHITE),
            BorderColor(Color::BLACK),
        ))
        .with_children(|cs| {
            for device_id in q_device_ids.devices.iter() {
                cs.spawn((
                    ChipButton,
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::WHITE),
                ))
                .with_children(|b| {
                    b.spawn((
                        Text::new(device_id),
                        TextFont {
                            font: common_assets.font.clone(),
                            font_size: 20.0,
                            font_smoothing: FontSmoothing::None,
                        },
                        TextColor(Color::BLACK),
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
                background_color.0 = Color::WHITE;
            }
            Interaction::Hovered => background_color.0 = GRAY.into(),
            Interaction::Pressed => {
                let device_id = button_text.0.clone();

                spawn_ev_writer.send(SpawnDeviceEvent {
                    device_id,
                    position: Position::ZERO,
                    init_drag: true,
                });
            }
        }
    }
}
