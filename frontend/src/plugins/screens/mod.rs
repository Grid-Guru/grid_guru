pub mod loading;
mod menu;

use crate::GameState;
use bevy::prelude::*;
use loading::LoadingPlugin;
use menu::MenuPlugin;

pub struct GridGuruScreenPlugin;
impl Plugin for GridGuruScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins((LoadingPlugin, MenuPlugin));
    }
}
