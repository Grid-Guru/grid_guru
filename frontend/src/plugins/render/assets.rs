use bevy::prelude::*;

pub struct RAssetsPlugin;
impl Plugin for RAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AllAssetHandles>();
        app.add_systems(Startup, load_asset);
    }
}

#[derive(Resource, Debug, Default)]
pub struct AllAssetHandles {
    pub rogue: Handle<Scene>,
    pub mage: Handle<Scene>,
    pub minion: Handle<Scene>,
    pub warrior: Handle<Scene>,

    pub dirt: Handle<Scene>,
    pub floor: Handle<Scene>,
    pub floor_detail: Handle<Scene>,
    pub barrel: Handle<Scene>,
    pub wall: Handle<Scene>,

    pub staff: Handle<Scene>,
    pub axe: Handle<Scene>,
    pub blade: Handle<Scene>,
    pub crossbow: Handle<Scene>,
    pub shield: Handle<Scene>,
}

fn load_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    // characters
    let rogue = "Skeleton_Rogue.glb";
    let mage = "Skeleton_Mage.glb";
    let minion = "Skeleton_Minion.glb";
    let warrior = "Skeleton_Warrior.glb";

    // blocks
    let dirt = "dirt.glb";
    let floor = "floor.glb";
    let floor_detail = "floor-detail.glb";
    let barrel = "barrel.glb";
    let wall = "wall.glb";

    // weapons
    let staff = "Skeleton_Staff.gltf";
    let axe = "Skeleton_Axe.gltf";
    let blade = "Skeleton_Blade.gltf";
    let shield = "Skeleton_shield_Large_A.gltf";
    let crossbow = "Skeleton_Crossbow.gltf";

    let all_asset_handles = AllAssetHandles {
        rogue: asset_server.load(GltfAssetLabel::Scene(0).from_asset(rogue)),
        mage: asset_server.load(GltfAssetLabel::Scene(0).from_asset(mage)),
        minion: asset_server.load(GltfAssetLabel::Scene(0).from_asset(minion)),
        warrior: asset_server.load(GltfAssetLabel::Scene(0).from_asset(warrior)),

        dirt: asset_server.load(GltfAssetLabel::Scene(0).from_asset(dirt)),
        floor: asset_server.load(GltfAssetLabel::Scene(0).from_asset(floor)),
        floor_detail: asset_server.load(GltfAssetLabel::Scene(0).from_asset(floor_detail)),
        barrel: asset_server.load(GltfAssetLabel::Scene(0).from_asset(barrel)),
        wall: asset_server.load(GltfAssetLabel::Scene(0).from_asset(wall)),

        staff: asset_server.load(GltfAssetLabel::Scene(0).from_asset(staff)),
        axe: asset_server.load(GltfAssetLabel::Scene(0).from_asset(axe)),
        blade: asset_server.load(GltfAssetLabel::Scene(0).from_asset(blade)),
        shield: asset_server.load(GltfAssetLabel::Scene(0).from_asset(shield)),
        crossbow: asset_server.load(GltfAssetLabel::Scene(0).from_asset(crossbow)),
    };

    commands.insert_resource(all_asset_handles);
}
