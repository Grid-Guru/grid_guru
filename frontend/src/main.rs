use bevy::app::App;
use frontend::GamePlugin;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
