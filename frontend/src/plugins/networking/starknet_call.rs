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

use super::config::{
    GAME_ACTIONS_CONTRACT_ADDRESS, PLAYER_ONE_ADDRESS, PLAYER_ONE_PRIVATE_KEY, PLAYER_TWO_ADDRESS,
    PLAYER_TWO_PRIVATE_KEY, RPC_URL,
};
use super::tokio::{TokioRuntimeResource, TokioRuntimeState};

pub struct StarknetPlugin;
impl Plugin for StarknetPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<StarknetServerState>();
        app.insert_resource(ClaimTilePosition {
            x: "10",
            y: "10",
            current_selection: false,
        });
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
}

pub enum StarknetCommands {
    SetAccountPlayerOne,
    SetAccountPlayerTwo,
    SendCreateGameTx,
    SendJoinGameTx,
    SendClaimTileTx(&'static str, &'static str),
}

#[derive(Resource)]
pub struct ClaimTilePosition {
    pub x: &'static str,
    pub y: &'static str,
    pub current_selection: bool,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum StarknetServerState {
    #[default]
    NotReady,
    Ready,
}

fn handle_player_inputs(
    channel: Res<StarknetChannel>,
    input: Res<ButtonInput<KeyCode>>,
    mut claim_resource: ResMut<ClaimTilePosition>,
) {
    let keys = input.get_just_pressed();
    let mut modify_inputs = false;
    let mut value = "10";

    for key in keys {
        match key {
            KeyCode::Digit0 => {
                modify_inputs = true;
                value = "0";
            }
            KeyCode::Digit1 => {
                modify_inputs = true;
                value = "1";
            }
            KeyCode::Digit2 => {
                modify_inputs = true;
                value = "2";
            }
            KeyCode::Digit3 => {
                modify_inputs = true;
                value = "3";
            }
            KeyCode::Digit4 => {
                modify_inputs = true;
                value = "4";
            }
            KeyCode::Digit5 => {
                modify_inputs = true;
                value = "5";
            }
            KeyCode::Digit6 => {
                modify_inputs = true;
                value = "6";
            }
            KeyCode::Digit7 => {
                modify_inputs = true;
                value = "7";
            }
            KeyCode::KeyX => {
                claim_resource.current_selection = false;
            }
            KeyCode::KeyY => {
                claim_resource.current_selection = true;
            }
            KeyCode::KeyQ => {
                let _ = channel.tx.try_send(StarknetCommands::SetAccountPlayerOne);
            }
            KeyCode::KeyW => {
                let _ = channel.tx.try_send(StarknetCommands::SetAccountPlayerTwo);
            }
            KeyCode::KeyN => {
                let _ = channel.tx.try_send(StarknetCommands::SendCreateGameTx);
            }
            KeyCode::KeyJ => {
                let _ = channel.tx.try_send(StarknetCommands::SendJoinGameTx);
            }
            KeyCode::Space => {
                let _ = channel.tx.try_send(StarknetCommands::SendClaimTileTx(
                    claim_resource.x,
                    claim_resource.y,
                ));
            }
            _ => {}
        }

        if modify_inputs {
            if claim_resource.current_selection {
                claim_resource.y = value;
                info!("changed y selection to {value}");
            } else {
                claim_resource.x = value;
                info!("changed x selection to {value}");
            }
        }
    }
}

fn spawn_starknet_caller_thread(
    mut commands: Commands,
    rt: Res<TokioRuntimeResource>,
    mut next_state: ResMut<NextState<StarknetServerState>>,
) {
    let (tx, mut rx) = mpsc::channel::<StarknetCommands>(64);

    let _ = rt.0.spawn(async move {
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
                StarknetCommands::SendClaimTileTx(x, y) => {
                    let res = send_claim_tile_tx(&account, x, y).await;
                    match res {
                        Ok(tx) => info!("{tx:?}"),
                        Err(e) => info!("{e:?}"),
                    }
                }
            }
        }
    });

    commands.insert_resource(StarknetChannel { tx });
    next_state.set(StarknetServerState::Ready);
}

async fn get_rpc_provider() -> JsonRpcClient<HttpTransport> {
    let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(RPC_URL).unwrap()));

    provider
}

fn get_player1_account() -> (LocalWallet, Felt) {
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex(PLAYER_ONE_PRIVATE_KEY).unwrap(),
    ));

    let address = Felt::from_hex(PLAYER_ONE_ADDRESS).unwrap();

    (signer, address)
}

fn get_player2_account() -> (LocalWallet, Felt) {
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex(PLAYER_TWO_PRIVATE_KEY).unwrap(),
    ));

    let address = Felt::from_hex(PLAYER_TWO_ADDRESS).unwrap();

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
            to: Felt::from_hex(GAME_ACTIONS_CONTRACT_ADDRESS).unwrap(),
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
            to: Felt::from_hex(GAME_ACTIONS_CONTRACT_ADDRESS).unwrap(),
            selector: get_selector_from_name("join_game").unwrap(),
            calldata: vec![Felt::from_hex_unchecked("0x1")],
        }])
        .send()
        .await;

    tx
}

async fn send_claim_tile_tx(
    account: &SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    x: &'static str,
    y: &'static str,
) -> Result<
    starknet::core::types::InvokeTransactionResult,
    starknet::accounts::AccountError<
        starknet::accounts::single_owner::SignError<starknet::signers::local_wallet::SignError>,
    >,
> {
    let calldata = vec![
        Felt::from_hex_unchecked("0x1"),
        Felt::from_hex_unchecked(x),
        Felt::from_hex_unchecked(y),
    ];
    let tx = account
        .execute_v3(vec![Call {
            to: Felt::from_hex(GAME_ACTIONS_CONTRACT_ADDRESS).unwrap(),
            selector: get_selector_from_name("claim_tile").unwrap(),
            calldata,
        }])
        .send()
        .await;

    tx
}
