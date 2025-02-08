use bevy::prelude::*;

use super::{assets::AssetHandler, constants::MULTIPLIER};

pub struct RTilePlugin;
impl Plugin for RTilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_tiles);
    }
}

fn spawn_tiles(mut commands: Commands, asset_handler: Res<AssetHandler>) {
    let grid_array = (7, 7);
    for i in 0..grid_array.0 {
        for j in 0..grid_array.1 {
            let dirt_block = SceneRoot(asset_handler.dirt_block.clone());
            let transform =
                Transform::from_xyz((i as f32) * MULTIPLIER, (j as f32) * MULTIPLIER, 0.0);
            commands.spawn((dirt_block, transform));
        }
    }
}
