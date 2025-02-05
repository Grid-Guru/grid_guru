use bevy::prelude::*;

mod camera;
mod debug;
mod window;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((window::plugin, camera::plugin));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(debug::plugin);
    }
}
