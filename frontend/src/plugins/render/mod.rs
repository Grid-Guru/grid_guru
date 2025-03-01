mod assets;
mod camera;
mod constants;
mod highlight;
mod mouse;
mod player;
mod tile;

use crate::GameState;
use assets::RAssetsPlugin;
use bevy::prelude::*;
use camera::RCameraPlugin;
use highlight::HighlightPlugin;
use mouse::RMousePlugin;
use player::RPlayerPlugin;
use tile::RTilePlugin;

pub struct GridGuruRenderPlugin;
impl Plugin for GridGuruRenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            RAssetsPlugin,
            RCameraPlugin,
            RTilePlugin,
            RPlayerPlugin,
            RMousePlugin,
            HighlightPlugin,
        ));
    }
}
