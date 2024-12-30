use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct CommonAssets {
    #[asset(path = "fonts/VCR_OSD_MONO.ttf")]
    pub font: Handle<Font>,
}
