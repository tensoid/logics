use bevy::prelude::*;
use bevy_asset_loader::{
    asset_collection::AssetCollectionApp,
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
};
use common_assets::CommonAssets;
use designer_assets::DesignerAssets;

pub mod common_assets;
pub mod designer_assets;

//UNSURE: rename because of name clash
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        // app.add_loading_state(
        //     LoadingState::new(AssetLoadingState::Loading)
        //         .continue_to_state(AssetLoadingState::Done)
        //         .load_collection::<DesignerAssets>()
        //         .load_collection::<CommonAssets>(),
        // );

        // app.init_state::<AssetLoadingState>();

        //HACK: make this work with loading state
        app.init_collection::<DesignerAssets>();
        app.init_collection::<CommonAssets>();
    }
}

// #[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
// enum AssetLoadingState {
//     #[default]
//     Loading,
//     Done,
// }
