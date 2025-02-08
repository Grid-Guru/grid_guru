use bevy::prelude::*;

use super::{assets::AssetHandler, constants::MULTIPLIER};

pub struct RPlayerPlugin;
impl Plugin for RPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player);
    }
}

pub const X_POS: f32 = 1.0;
pub const Y_POS: f32 = 1.0;
pub const X_DIFF: f32 = 0.0;
pub const Y_DIFF: f32 = 0.4;
pub const Z_HEIGHT: f32 = 0.5;
pub const SCALE: f32 = 0.6;
pub const ANGLE_90: f32 = 1.570796;

fn spawn_player(mut commands: Commands, asset_handler: Res<AssetHandler>) {
    let player = SceneRoot(asset_handler.player1.clone());
    // let transform = Transform::from_xyz((7 as f32) * MULTIPLIER, (7 as f32) * MULTIPLIER, 0.0)
    //     .with_scale(Vec3::splat(0.5));
    // commands.spawn((player.clone(), transform));

    let transform = Transform::from_xyz(
        (X_POS + X_DIFF) * MULTIPLIER,
        (Y_POS + Y_DIFF) * MULTIPLIER,
        Z_HEIGHT,
    )
    .with_scale(Vec3::splat(SCALE))
    .with_rotation(Quat::from_axis_angle(Vec3::X, ANGLE_90));
    // .with_rotation(Quat::from_axis_angle(Vec3::Z, ANGLE_90));
    commands.spawn((player, transform));
}
