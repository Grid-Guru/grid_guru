pub mod config;
pub mod starknet_call;
pub mod tokio;
pub mod torii;

use bevy::prelude::*;
use starknet_call::StarknetPlugin;
use tokio::TokioPlugin;
use torii::ToriiPlugin;

pub struct NetworkingPlugin;
impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StarknetPlugin);
        app.add_plugins(ToriiPlugin);
        app.add_plugins(TokioPlugin);
    }
}
