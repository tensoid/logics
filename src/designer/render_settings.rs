use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct CircuitBoardRenderingSettings {
    pub background_color: Color,
    pub signal_high_color: Color,
    pub signal_low_color: Color,
    pub chip_pin_gap: f32,
    pub chip_pin_radius: f32,
    pub chip_color: Color,
    pub binary_io_pin_radius: f32,
    pub binary_io_display_color: Color,
    pub binary_io_switch_color: Color,
    pub binary_io_display_extents: Vec2,
    pub binary_io_switch_extents: Vec2,
    pub wire_line_width: f32,
}

pub fn init_render_settings(app: &mut App) {
    let render_settings = CircuitBoardRenderingSettings {
        background_color: Color::rgb(0.1, 0.1, 0.1),
        signal_low_color: Color::BLACK,
        signal_high_color: Color::GREEN,
        chip_pin_gap: 25.0,
        chip_pin_radius: 7.0,
        chip_color: Color::WHITE,
        binary_io_pin_radius: 7.0,
        binary_io_display_color: Color::WHITE,
        binary_io_switch_color: Color::WHITE,
        binary_io_display_extents: Vec2::new(30.0, 30.0),
        binary_io_switch_extents: Vec2::new(30.0, 30.0),
        wire_line_width: 2.0,
    };

    app.insert_resource(render_settings.clone())
        .insert_resource(ClearColor(render_settings.background_color));
}
