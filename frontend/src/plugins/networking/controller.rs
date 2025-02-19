use account_sdk::controller::Controller;
use bevy::prelude::*;
use starknet::{
    core::types::Felt,
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider, Url},
    signers::SigningKey,
};

use super::config::{PLAYER_ONE_PRIVATE_KEY, RPC_URL};

pub struct ControllerPlugin;
impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {}
}

async fn start_controller() {
    let owner = SigningKey::from_secret_scalar(Felt::from_hex_unchecked(PLAYER_ONE_PRIVATE_KEY));
    let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(RPC_URL).unwrap()));
    let chain_id = provider.chain_id().await.unwrap();
    let app_id = "https://app.gridguru.xyz";
    let class_hash = "0x1";

    // let username = "player_one";
    // let controller = Controller::new(
    //     app_id.to_string(),
    //     username.to_string(),
    //     Felt::from_hex_unchecked(class_hash),
    //     Url::parse(RPC_URL).unwrap(),
    //     owner,
    //     provider,
    //     chain_id,
    // );
}
