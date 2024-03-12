use bevy::prelude::*;

/// Tracks wether controls should be enabled for the designer at the given time.
/// If for example a UI is open it should be inactive.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum DesignerState {
    #[default]
    Active,
    Inactive,
}

//TODO: pull designer state logic into system here
