mod plugins;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use plugins::{
    dojo_models::DojoModelsPlugin, networking::NetworkingPlugin, render::GridGuruRenderPlugin,
    screens::GridGuruScreenPlugin,
};

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    Playing,
    Menu,
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Index(1)),
                ..default()
            }),
            ..default()
        }));
        app.add_plugins(WorldInspectorPlugin::new());
        app.add_plugins(GridGuruScreenPlugin);
        app.add_plugins(NetworkingPlugin);
        app.add_plugins(DojoModelsPlugin);
        app.add_plugins(GridGuruRenderPlugin);
    }
}
