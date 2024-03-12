use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct CircuitBoardRenderingSettings {
    pub background_color: Color,
    pub signal_high_color: Color,
    pub signal_low_color: Color,
    pub chip_pin_gap: f32,
    pub chip_pin_radius: f32,
    pub chip_color: Color,
    pub board_entity_stroke_color: Color,
    pub board_entity_stroke_color_selected: Color,
    pub board_entity_stroke_width: f32,
    pub binary_io_pin_radius: f32,
    pub binary_io_color: Color,
    pub wire_line_width: f32,
    pub pin_color: Color,
    pub hovered_pin_color: Color,
    pub selection_box_stroke_color: Color,
    pub selection_box_stroke_width: f32,
    pub selection_box_fill_color: Color,
}

pub fn init_render_settings(app: &mut App) {
    // LIGHT-MODE
    let render_settings = CircuitBoardRenderingSettings {
        background_color: Color::rgb(0.42, 0.45, 0.63),
        signal_low_color: Color::BLACK,
        signal_high_color: Color::GREEN,
        chip_pin_gap: 25.0,
        chip_pin_radius: 7.0,
        chip_color: Color::WHITE,
        board_entity_stroke_color: Color::BLACK,
        board_entity_stroke_color_selected: Color::BLUE,
        board_entity_stroke_width: 2.0,
        binary_io_pin_radius: 7.0,
        binary_io_color: Color::WHITE,
        wire_line_width: 2.0,
        pin_color: Color::BLACK,
        hovered_pin_color: Color::rgb(0.4, 0.4, 0.4),
        selection_box_fill_color: Color::rgba(1.0, 1.0, 1.0, 0.1),
        selection_box_stroke_width: 1.0,
        selection_box_stroke_color: Color::BLACK,
    };

    app.insert_resource(render_settings.clone())
        .insert_resource(ClearColor(render_settings.background_color));
}
