use bevy::prelude::*;

#[derive(Resource)]
pub struct DesignerAssets {
    pub font: Handle<Font>,
    pub binary_switch_image: Handle<Image>,
}

pub fn load_assets(asset_server: ResMut<AssetServer>, mut commands: Commands) {
    commands.insert_resource(DesignerAssets {
        font: asset_server.load("fonts/VCR_OSD_MONO.ttf"),
        binary_switch_image: asset_server.load("images/switch.png"),
    });
}
