use bevy::prelude::*;

/// Tracks wether controls should be enabled for the designer at the given time.
/// If for example a UI is open it should be inactive.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
#[allow(dead_code)]
pub enum DesignerState {
    #[default]
    Active,
    Inactive,
}
