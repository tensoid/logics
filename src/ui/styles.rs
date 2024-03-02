use bevy::prelude::*;

pub fn empty_board_tooltip_style() -> Style {
    Style {
        justify_self: JustifySelf::Center,
        align_self: AlignSelf::Center,
        display: Display::Flex,
        ..default()
    }
}

pub fn empty_board_tooltip_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    let font: Handle<Font> = asset_server.load("fonts/VCR_OSD_MONO.ttf");

    TextStyle {
        font,
        font_size: 25.0,
        ..default()
    }
}

pub fn chip_selector_style() -> Style {
    Style {
        width: Val::Vw(20.0),
        height: Val::Vh(40.0),
        align_self: AlignSelf::Center,
        justify_self: JustifySelf::Center,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(10.0),
        ..default()
    }
}

pub fn chip_selector_background_color() -> BackgroundColor {
    Color::BLACK.into()
}

pub fn chip_button_style() -> Style {
    Style {
        width: Val::Percent(50.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn chip_button_background_color() -> BackgroundColor {
    Color::BLACK.into()
}

pub fn chip_button_background_color_hovered() -> BackgroundColor {
    Color::GRAY.into()
}

pub fn chip_button_background_color_pressed() -> BackgroundColor {
    Color::GRAY.into()
}

pub fn chip_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    let font: Handle<Font> = asset_server.load("fonts/VCR_OSD_MONO.ttf");

    TextStyle {
        font: font.clone(),
        font_size: 20.0,
        ..default()
    }
}
