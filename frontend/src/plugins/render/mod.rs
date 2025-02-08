mod assets;
mod camera;
mod constants;
mod player;
mod tile;

use assets::RAssetsPlugin;
use bevy::prelude::*;
use camera::RCameraPlugin;
use player::RPlayerPlugin;
use tile::RTilePlugin;

pub struct GridGuruRenderPlugin;
impl Plugin for GridGuruRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RCameraPlugin);
        app.add_plugins(RAssetsPlugin);
        app.add_plugins(RTilePlugin);
        app.add_plugins(RPlayerPlugin);
    }
}
