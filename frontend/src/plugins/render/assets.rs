use bevy::prelude::*;

pub struct RAssetsPlugin;
impl Plugin for RAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetHandler>();
        app.add_systems(Startup, load_asset);
    }
}

#[derive(Resource, Debug, Default)]
pub struct AssetHandler {
    pub dirt_block: Handle<Scene>,
    pub player1: Handle<Scene>,
}

fn load_asset(asset_server: Res<AssetServer>, mut asset_handler: ResMut<AssetHandler>) {
    let tile_asset_path = "kenney_minidungeon\\Models\\glb\\dirt.glb";
    let player1_path = "kaykit_skeletons\\characters\\gltf\\Skeleton_Rogue.glb";

    let tile_handle: Handle<Scene> =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset(tile_asset_path));
    let player1_handle: Handle<Scene> =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset(player1_path));

    asset_handler.dirt_block = tile_handle;
    asset_handler.player1 = player1_handle;
}
