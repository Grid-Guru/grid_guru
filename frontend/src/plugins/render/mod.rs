mod camera;
mod tile;

use bevy::prelude::*;
use camera::RCameraPlugin;
use tile::RPlayerPlugin;

pub struct GridGuruRenderPlugin;
impl Plugin for GridGuruRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RCameraPlugin);
        app.add_plugins(RPlayerPlugin);
    }
}
