use bevy::prelude::*;

pub struct RPlayerPlugin;
impl Plugin for RPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_tile);
    }
}

fn setup_tile(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _character_asset_path = "kaykit_skeletons\\characters\\gltf\\Skeleton_Mage.glb";
    let tile_asset_path = "kenney_minidungeon\\Models\\glb\\wall.glb";

    let my_gltf =
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(tile_asset_path)));

    commands.spawn(my_gltf);
}
