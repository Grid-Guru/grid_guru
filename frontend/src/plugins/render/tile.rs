use bevy::prelude::*;

use super::assets::AllAssetHandles;
use super::constants::{XMUL, YMUL};
use super::highlight::make_tiles_highlightable;
use crate::GameState;

pub struct RTilePlugin;
impl Plugin for RTilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_tiles)
            .register_type::<Tile>()
            // Add system to make tiles highlightable after they're spawned
            .add_systems(
                OnEnter(GameState::Playing),
                make_tiles_highlightable.after(spawn_tiles),
            );
    }
}

// Component to mark an entity as a tile and track its grid position
#[derive(Component, Debug, Reflect)]
pub struct Tile {
    pub grid_x: u32,
    pub grid_y: u32,
}

fn spawn_tiles(mut commands: Commands, asset_handler: Res<AllAssetHandles>) {
    let grid_array = (8, 8);
    for i in 0..grid_array.0 {
        for j in 0..grid_array.1 {
            let dirt_block = SceneRoot(asset_handler.dirt.clone());
            let transform = Transform::from_xyz(i as f32 * XMUL, j as f32 * YMUL, 0.0);

            // Spawn tile with Tile component to track its grid position
            commands.spawn((
                dirt_block,
                transform,
                Tile {
                    grid_x: i,
                    grid_y: j,
                },
                Name::new(format!("Tile ({}, {})", i, j)),
            ));
        }
    }
}
