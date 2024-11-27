use bevy::{
    color::palettes::css::{BLACK, BLUE, LIME, RED, WHITE},
    prelude::*,
};

#[derive(Resource, Clone)]
pub struct CircuitBoardRenderingSettings {
    pub background_color: Color,
    pub signal_high_color: Color,
    pub signal_low_color: Color,
    pub signal_conflict_color: Color,
    pub chip_pin_gap: f32,
    pub chip_pin_radius: f32,
    pub chip_pin_label_font_size: f32,
    pub chip_label_font_size: f32,
    pub chip_color: Color,
    pub device_stroke_color: Color,
    pub device_stroke_color_selected: Color,
    pub device_stroke_width: f32,
    pub device_edge_radius: f32,
    pub device_io_pin_radius: f32,
    pub binary_io_color: Color,
    pub binary_switch_extents: Vec2,
    pub binary_display_extents: Vec2,
    pub binary_display_font_size: f32,
    pub clock_extents: Vec2,
    pub clock_color: Color,
    pub clock_label_font_size: f32,
    pub clock_pin_radius: f32,
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
        background_color: Color::srgb(0.42, 0.45, 0.63),
        signal_low_color: BLACK.into(),
        signal_high_color: LIME.into(),
        signal_conflict_color: RED.into(),
        chip_pin_gap: 25.0,
        chip_pin_radius: 7.0,
        chip_pin_label_font_size: 10.0,
        chip_label_font_size: 15.0,
        chip_color: WHITE.into(),
        device_stroke_color: BLACK.into(),
        device_stroke_color_selected: BLUE.into(),
        device_stroke_width: 2.0,
        device_edge_radius: 5.0,
        device_io_pin_radius: 7.0,
        binary_io_color: WHITE.into(),
        binary_switch_extents: Vec2::new(60.0, 30.0),
        binary_display_extents: Vec2::new(30.0, 30.0),
        binary_display_font_size: 15.0,
        clock_extents: Vec2::new(30.0, 30.0),
        clock_color: WHITE.into(),
        clock_label_font_size: 15.0,
        clock_pin_radius: 7.0,
        wire_line_width: 4.0,
        pin_color: BLACK.into(),
        hovered_pin_color: Color::srgb(0.4, 0.4, 0.4),
        selection_box_fill_color: Color::srgba(1.0, 1.0, 1.0, 0.1),
        selection_box_stroke_width: 1.0,
        selection_box_stroke_color: BLACK.into(),
    };

    app.insert_resource(render_settings.clone())
        .insert_resource(ClearColor(render_settings.background_color));
}
