use bevy::prelude::*;

use crate::{
    designer::{board_entity::BoardEntity, chip::ChipSpecs},
    events::events::{OpenChipSelectorEvent, SpawnChipEvent, SpawnIOPinEvent},
};

use super::styles::*;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum ChipSelectorState {
    #[default]
    Closed,
    Open,
}

#[derive(Component)]
pub struct ChipSelector;

#[derive(Component)]
pub struct ChipButton;

#[derive(Component)]
pub struct EmptyBoardTooltip;

pub fn spawn_empty_board_tooltip(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        EmptyBoardTooltip,
        TextBundle::from_section(
            "Press 'Space' to spawn chips and components.",
            empty_board_tooltip_text_style(&asset_server),
        )
        .with_style(empty_board_tooltip_style()),
    ));
}

pub fn update_emtpy_board_tooltip(
    q_board_entities: Query<(), With<BoardEntity>>,
    mut q_tooltip: Query<&mut Style, With<EmptyBoardTooltip>>,
) {
    if let Ok(mut tooltip) = q_tooltip.get_single_mut() {
        if q_board_entities.iter().len() > 0 {
            tooltip.display = Display::None;
        } else {
            tooltip.display = Display::Flex;
        }
    }
}

//TODO: make exit button or escape or something
pub fn toggle_chip_selector(
    chip_selector_state: Res<State<ChipSelectorState>>,
    mut chip_selector_next_state: ResMut<NextState<ChipSelectorState>>,
    mut open_chip_selector_ev: EventReader<OpenChipSelectorEvent>,
) {
    for _ in open_chip_selector_ev.read() {
        let next_state = match chip_selector_state.get() {
            ChipSelectorState::Closed => ChipSelectorState::Open,
            ChipSelectorState::Open => ChipSelectorState::Closed,
        };
        chip_selector_next_state.set(next_state);
    }
}

pub fn spawn_chip_selector(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_chip_specs: Res<ChipSpecs>,
) {
    commands
        .spawn((
            ChipSelector,
            NodeBundle {
                style: chip_selector_style(),
                background_color: chip_selector_background_color(),
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
    mut chip_ev_writer: EventWriter<SpawnChipEvent>,
    mut io_pin_ev_writer: EventWriter<SpawnIOPinEvent>,
    mut chip_selector_next_state: ResMut<NextState<ChipSelectorState>>,
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

                chip_selector_next_state.set(ChipSelectorState::Closed);

                let chip_name = button_text.sections.first().unwrap().value.clone();

                if chip_name == "PORT-IN" {
                    io_pin_ev_writer.send(SpawnIOPinEvent {
                        is_input: true,
                        position: Vec2::ZERO,
                    });
                } else if chip_name == "PORT-OUT" {
                    io_pin_ev_writer.send(SpawnIOPinEvent {
                        is_input: false,
                        position: Vec2::ZERO,
                    });
                } else {
                    chip_ev_writer.send(SpawnChipEvent {
                        chip_name,
                        position: Vec2::ZERO,
                    });
                }
            }
        }
    }
}
