mod plugins;

use bevy::{log::LogPlugin, prelude::*, state::app::StatesPlugin};
use plugins::{dojo_models::DojoModelsPlugin, tokio::TokioPlugin, torii::ToriiPlugin};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MinimalPlugins);
        app.add_plugins(StatesPlugin);
        app.add_plugins(LogPlugin::default());
        app.add_plugins(TokioPlugin);
        app.add_plugins(ToriiPlugin);
        app.add_plugins(DojoModelsPlugin);
    }
}
