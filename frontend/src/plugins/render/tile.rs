use bevy::prelude::*;

use super::assets::AllAssetHandles;

pub struct RTilePlugin;
impl Plugin for RTilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_tiles);
    }
}

pub const YMUL: f32 = 1.2;
pub const XMUL: f32 = 1.2;

fn spawn_tiles(mut commands: Commands, asset_handler: Res<AllAssetHandles>) {
    let grid_array = (8, 8);
    for i in 0..grid_array.0 {
        for j in 0..grid_array.1 {
            let dirt_block = SceneRoot(asset_handler.dirt.clone());
            let transform = Transform::from_xyz(i as f32 * XMUL, j as f32 * YMUL, 0.0);
            commands.spawn((dirt_block, transform));
        }
    }
}
