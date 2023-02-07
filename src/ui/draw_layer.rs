pub enum DrawLayer {
    Grid,
    Chip,
    ChipName,
    Pin,
    Wire,
}

impl DrawLayer {
    pub fn get_z(self) -> f32 {
        match self {
            DrawLayer::Grid => 0.0,
            DrawLayer::Chip => 0.1,
            DrawLayer::ChipName => 0.2,
            DrawLayer::Pin => 0.3,
            DrawLayer::Wire => 0.4,
        }
    }
}
