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
use tokio::{sync::mpsc, task::AbortHandle};

use super::tokio::{TokioRuntimeResource, TokioRuntimeState};

pub struct StarknetPlugin;
impl Plugin for StarknetPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<StarknetServerState>();
        app.add_systems(
            OnEnter(TokioRuntimeState::Ready),
            spawn_starknet_caller_thread,
        );
        app.add_systems(
            Update,
            handle_player_inputs.run_if(in_state(StarknetServerState::Ready)),
        );
    }
}

#[derive(Resource)]
pub struct StarknetChannel {
    tx: mpsc::Sender<StarknetCommands>,
    abort_handle: AbortHandle,
}

pub enum StarknetCommands {
    SetAccountPlayerOne,
    SetAccountPlayerTwo,
    SendCreateGameTx,
    SendJoinGameTx,
    SendClaimTileTx,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum StarknetServerState {
    #[default]
    NotReady,
    Ready,
}

fn handle_player_inputs(channel: Res<StarknetChannel>, input: Res<ButtonInput<KeyCode>>) {
    let keys = input.get_just_pressed();
    for key in keys {
        match key {
            KeyCode::Digit1 => {
                let _ = channel.tx.try_send(StarknetCommands::SetAccountPlayerOne);
            }
            KeyCode::Digit2 => {
                let _ = channel.tx.try_send(StarknetCommands::SetAccountPlayerTwo);
            }
            KeyCode::KeyN => {
                let _ = channel.tx.try_send(StarknetCommands::SendCreateGameTx);
            }
            KeyCode::KeyJ => {
                let _ = channel.tx.try_send(StarknetCommands::SendJoinGameTx);
            }
            _ => {}
        }
    }
}

fn spawn_starknet_caller_thread(
    mut commands: Commands,
    rt: Res<TokioRuntimeResource>,
    mut next_state: ResMut<NextState<StarknetServerState>>,
) {
    let (tx, mut rx) = mpsc::channel::<StarknetCommands>(64);

    let abort_handle =
        rt.0.spawn(async move {
            let provider = get_rpc_provider().await;
            let (signer, address) = get_player1_account();
            let chain_id = provider.chain_id().await.unwrap();

            let mut account = create_player_account(provider, signer, address, chain_id);

            info!("Started STARKNET TX SENDING SERVER...");
            while let Some(starknet_command) = rx.recv().await {
                match starknet_command {
                    StarknetCommands::SetAccountPlayerOne => {
                        let provider = get_rpc_provider().await;
                        let (signer, address) = get_player1_account();
                        let chain_id = provider.chain_id().await.unwrap();

                        account = create_player_account(provider, signer, address, chain_id);
                        info!("Switched to player one.");
                    }
                    StarknetCommands::SetAccountPlayerTwo => {
                        let provider = get_rpc_provider().await;
                        let (signer, address) = get_player2_account();
                        let chain_id = provider.chain_id().await.unwrap();

                        account = create_player_account(provider, signer, address, chain_id);
                        info!("Switched to player two.");
                    }
                    StarknetCommands::SendCreateGameTx => {
                        let res = send_create_game_tx(&account).await;
                        info!("Sending a create_game transaction.");
                        match res {
                            Ok(tx) => info!("{tx:?}"),
                            Err(e) => info!("{e:?}"),
                        }
                    }
                    StarknetCommands::SendJoinGameTx => {
                        let res = send_join_game_tx(&account).await;
                        match res {
                            Ok(tx) => info!("{tx:?}"),
                            Err(e) => info!("{e:?}"),
                        }
                    }
                    _ => {}
                }
            }
        })
        .abort_handle();

    commands.insert_resource(StarknetChannel { tx, abort_handle });
    next_state.set(StarknetServerState::Ready);
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

fn create_player_account(
    provider: JsonRpcClient<HttpTransport>,
    signer: LocalWallet,
    address: Felt,
    chain_id: Felt,
) -> SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet> {
    let account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id,
        starknet::accounts::ExecutionEncoding::New,
    );

    account
}

async fn send_create_game_tx(
    account: &SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
) -> Result<
    starknet::core::types::InvokeTransactionResult,
    starknet::accounts::AccountError<
        starknet::accounts::single_owner::SignError<starknet::signers::local_wallet::SignError>,
    >,
> {
    let tx = account
        .execute_v3(vec![Call {
            to: Felt::from_hex(
                "0x0499f2e2515b64f360601510e4bf47b904ecddc50a40ce6e461e1dd4d7389398",
            )
            .unwrap(),
            selector: get_selector_from_name("create_game").unwrap(),
            calldata: vec![],
        }])
        .send()
        .await;

    tx
}

async fn send_join_game_tx(
    account: &SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
) -> Result<
    starknet::core::types::InvokeTransactionResult,
    starknet::accounts::AccountError<
        starknet::accounts::single_owner::SignError<starknet::signers::local_wallet::SignError>,
    >,
> {
    let tx = account
        .execute_v3(vec![Call {
            to: Felt::from_hex(
                "0x0499f2e2515b64f360601510e4bf47b904ecddc50a40ce6e461e1dd4d7389398",
            )
            .unwrap(),
            selector: get_selector_from_name("join_game").unwrap(),
            calldata: vec![Felt::from_hex_unchecked("0x1")],
        }])
        .send()
        .await;

    tx
}
