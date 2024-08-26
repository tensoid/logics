use bevy::prelude::*;

use crate::{
    designer::{board_entity::Position, chip::BuiltinChips, designer_state::DesignerState},
    events::events::SpawnBoardEntityEvent,
};

use super::styles::*;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum ChipSelectorState {
    #[default]
    Open,
    Closed,
}

#[derive(Component)]
pub struct ChipSelector;

#[derive(Component)]
pub struct ChipButton;

pub fn spawn_chip_selector(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_chip_specs: Res<BuiltinChips>,
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
            for chip_name in q_chip_specs
                .0
                .iter()
                .map(|spec| spec.name.clone())
                .chain(vec!["PORT-IN".to_string(), "PORT-OUT".to_string()])
            {
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
                        chip_name,
                        chip_button_text_style(&asset_server),
                    ));
                });
            }
        });
}

pub fn despawn_chip_selector(
    mut commands: Commands,
    q_chip_selector: Query<Entity, With<ChipSelector>>,
) {
    for chip_selector_entity in q_chip_selector.iter() {
        commands.entity(chip_selector_entity).despawn_recursive();
    }
}

#[allow(clippy::type_complexity)]
pub fn chip_selector_button_interact(
    mut q_buttons: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<ChipButton>),
    >,
    q_texts: Query<&Text>,
    mut spawn_ev_writer: EventWriter<SpawnBoardEntityEvent>,
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

                let chip_name = button_text.sections.first().unwrap().value.clone();

                spawn_ev_writer.send(SpawnBoardEntityEvent {
                    name: chip_name,
                    position: Position(Vec2::ZERO),
                    init_drag: true,
                });
            }
        }
    }
}
