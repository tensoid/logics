use bevy::prelude::*;

#[derive(Resource)]
pub struct CircuitBoardRenderingSettings {
    pub signal_high_color: Color,
    pub signal_low_color: Color,
    pub chip_pin_gap: f32,
    pub chip_pin_radius: f32,
    pub binary_io_pin_radius: f32,
    pub wire_line_width: f32,
    pub binary_io_handlebar_width: f32,
    pub binary_io_handlebar_length: f32,
    pub binary_io_handlebar_color: Color, //TODO: shapes maybe
}

pub fn register_render_settings(app: &mut App) {
    app.insert_resource(CircuitBoardRenderingSettings {
        signal_low_color: Color::BLACK,
        signal_high_color: Color::GREEN,
        chip_pin_gap: 25.0,
        chip_pin_radius: 7.0,
        binary_io_pin_radius: 10.0,
        binary_io_handlebar_width: 8.0,
        binary_io_handlebar_length: 40.0,
        binary_io_handlebar_color: Color::BLACK,
        wire_line_width: 2.0,
    });
}
