use bevy::{asset::Handle, prelude::*};

use crate::plugins::{
    dojo_models::{Game, Tile},
    torii::BevyFelt,
};

use super::assets::AllAssetHandles;

pub struct RPlayerPlugin;
impl Plugin for RPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_player_markers.run_if(is_game_ready));
        app.add_systems(Update, animate_markers);
    }
}

pub const X_DIFF: f32 = 0.;
pub const X_MUL: f32 = 1.2;
pub const Y_MUL: f32 = 1.2;
pub const Y_DIFF: f32 = 0.5;
pub const Z_HEIGHT: f32 = 1.;
pub const SCALE: f32 = 1.;
pub const ANGLE_90: f32 = 1.570796;

#[derive(Component, Reflect)]
pub struct DerivePlayerPosition {
    pub owner: BevyFelt,
    pub x: u8,
    pub y: u8,
}

fn is_game_ready(game_query: Query<&Game>) -> bool {
    game_query.iter().count() > 0
}

#[derive(Component, Reflect)]
pub struct RenderedPlayerMarker {
    pub x: u8,
    pub y: u8,
}

fn spawn_player_markers(
    mut commands: Commands,
    asset_handler: Res<AllAssetHandles>,
    tile_query: Query<&Tile>,
    game_query: Query<&Game>,
    render_marker_query: Query<&mut RenderedPlayerMarker>,
) {
    let game = game_query.single();
    let p1 = game.player_one.clone();
    let _p2 = game.player_two.clone();

    for tile in tile_query.iter() {
        if let None = render_marker_query
            .iter()
            .find(|m| m.x == tile.x && m.y == tile.y)
        {
            let marker_handle: Handle<Scene>;
            if p1 == tile.owner {
                marker_handle = asset_handler.blade.clone();
            } else {
                marker_handle = asset_handler.shield.clone();
            }

            let marker_scene = SceneRoot(marker_handle);
            let transform = Transform::from_xyz(
                (tile.x as f32 * X_MUL) + X_DIFF,
                (tile.y as f32 * Y_MUL) + Y_DIFF,
                Z_HEIGHT,
            )
            .with_scale(Vec3::splat(SCALE))
            .with_rotation(Quat::from_axis_angle(Vec3::X, ANGLE_90));
            commands.spawn((
                marker_scene,
                transform,
                RenderedPlayerMarker {
                    x: tile.x,
                    y: tile.y,
                },
            ));
        }
    }
}

fn animate_markers(mut query: Query<&mut Transform, With<RenderedPlayerMarker>>, time: Res<Time>) {
    for mut marker in query.iter_mut() {
        marker.rotate_z(0.5 * time.delta_secs());
    }
}
