use bevy::prelude::*;

pub const DEFAULT_FONT: &str = "fonts/VCR_OSD_MONO.ttf";

pub fn chip_selector_style() -> Style {
    Style {
        width: Val::Vw(20.0),
        height: Val::Vh(100.0),
        justify_self: JustifySelf::End,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        border: UiRect::left(Val::Px(2.0)),
        ..default()
    }
}

pub fn chip_selector_background_color() -> BackgroundColor {
    Color::WHITE.into()
}

pub fn chip_selector_border_color() -> BorderColor {
    Color::BLACK.into()
}

pub fn chip_button_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn chip_button_background_color() -> BackgroundColor {
    Color::WHITE.into()
}

pub fn chip_button_background_color_hovered() -> BackgroundColor {
    Color::GRAY.into()
}

pub fn chip_button_background_color_pressed() -> BackgroundColor {
    Color::GRAY.into()
}

pub fn chip_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    let font: Handle<Font> = asset_server.load(DEFAULT_FONT);

    TextStyle {
        font: font.clone(),
        font_size: 20.0,
        color: Color::BLACK,
    }
}
