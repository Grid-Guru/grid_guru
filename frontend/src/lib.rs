use bevy::{app::Plugin, MinimalPlugins};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(MinimalPlugins);
    }
}
