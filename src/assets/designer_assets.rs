use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct DesignerAssets {
    #[asset(path = "images/switch.png")]
    pub binary_switch_image: Handle<Image>,
}
