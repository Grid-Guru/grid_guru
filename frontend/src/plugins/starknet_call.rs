use bevy::prelude::*;
use starknet::{
    accounts::{Account, SingleOwnerAccount},
    core::{
        types::{Call, Felt},
        utils::get_selector_from_name,
    },
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider, Url},
    signers::{LocalWallet, SigningKey},
};
use tokio::sync::mpsc;

use super::tokio::{TokioRuntimeResource, TokioRuntimeState};

pub struct StarknetPlugin;
impl Plugin for StarknetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_starknet_channel_resource);
        app.add_systems(
            OnEnter(TokioRuntimeState::Ready),
            spawn_starknet_caller_thread,
        );
    }
}

#[derive(Resource)]
pub struct StarknetChannel {
    rx: mpsc::Receiver<u8>,
    tx: mpsc::Sender<u8>,
}

fn setup_starknet_channel_resource(mut commands: Commands) {
    let (tx, rx) = mpsc::channel::<u8>(64);
    commands.insert_resource(StarknetChannel { rx, tx });
}

fn spawn_starknet_caller_thread(rt: Res<TokioRuntimeResource>, channel: Res<StarknetChannel>) {
    let _tx = channel.tx.clone();
    let _ = rt.0.spawn(async move {
        let provider = get_rpc_provider().await;
        let (signer, address) = get_player1_account();
        let chain_id = provider.chain_id().await.unwrap();
        info!("{chain_id}");
        let account = SingleOwnerAccount::new(
            provider,
            signer,
            address,
            chain_id,
            starknet::accounts::ExecutionEncoding::New,
        );
        info!("got a working starknet account! yippie!");

        let tx_res = account
            .execute_v3(vec![Call {
                to: Felt::from_hex(
                    "0x0499f2e2515b64f360601510e4bf47b904ecddc50a40ce6e461e1dd4d7389398",
                )
                .unwrap(),
                selector: get_selector_from_name("create_game").unwrap(),
                calldata: vec![],
            }])
            .send()
            .await
            .unwrap();

        info!("starknet tx result: {tx_res:?}");
    });
}

async fn get_rpc_provider() -> JsonRpcClient<HttpTransport> {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("http://127.0.0.1:5050").unwrap(),
    ));

    provider
}

fn get_player1_account() -> (LocalWallet, Felt) {
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("0xc5b2fcab997346f3ea1c00b002ecf6f382c5f9c9659a3894eb783c5320f912").unwrap(),
    ));

    let address =
        Felt::from_hex("0x127fd5f1fe78a71f8bcd1fec63e3fe2f0486b6ecd5c86a0466c3a21fa5cfcec")
            .unwrap();

    (signer, address)
}

fn get_player2_account() -> (LocalWallet, Felt) {
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("0x33003003001800009900180300d206308b0070db00121318d17b5e6262150b").unwrap(),
    ));

    let address =
        Felt::from_hex("0x5b6b8189bb580f0df1e6d6bec509ff0d6c9be7365d10627e0cf222ec1b47a71")
            .unwrap();

    (signer, address)
}
