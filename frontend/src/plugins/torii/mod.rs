pub mod new_torii;

use super::tokio::{TokioRuntimeResource, TokioRuntimeState};
use bevy::prelude::*;
use starknet::core::types::Felt;
use std::sync::Arc;
use tokio::sync::oneshot;
use torii_client::client::Client;
use torii_grpc::types::Query as ToriiQuery;

pub struct ToriiPlugin;
impl Plugin for ToriiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ToriiConnectionState>();
        app.add_systems(
            Update,
            try_connection.run_if(
                in_state(TokioRuntimeState::Ready)
                    .and(not(in_state(ToriiConnectionState::Connected))),
            ),
        );
        app.add_systems(OnEnter(ToriiConnectionState::Connected), query_all_entities);
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
enum ToriiConnectionState {
    #[default]
    Disconnected,
    Connecting,
    Connected,
}

#[derive(Component)]
struct OneshotClientRx(oneshot::Receiver<Client>);

#[derive(Resource)]
struct ConnectedClient(Arc<Client>);

fn try_connection(
    mut commands: Commands,
    rt: Res<TokioRuntimeResource>,
    mut rx_query: Query<&mut OneshotClientRx>,
    state: Res<State<ToriiConnectionState>>,
    mut next_state: ResMut<NextState<ToriiConnectionState>>,
) {
    match state.get() {
        ToriiConnectionState::Disconnected => {
            let (tx, rx) = oneshot::channel::<Client>();
            rt.0.spawn(attempt_torii_client_connection(tx));
            commands.spawn(OneshotClientRx(rx));
            next_state.set(ToriiConnectionState::Connecting);
        }
        ToriiConnectionState::Connecting => {
            for mut rx in rx_query.iter_mut() {
                if let Ok(client) = rx.0.try_recv() {
                    let arc = Arc::new(client);
                    commands.insert_resource(ConnectedClient(arc));
                    next_state.set(ToriiConnectionState::Connected);
                }
            }
        }
        _ => {}
    }
}

async fn attempt_torii_client_connection(tx: oneshot::Sender<Client>) {
    let torii_url = "http://localhost:8080".to_string();
    let rpc_url = "http://127.0.0.1:5050".to_string();
    let relay_url = "/ip4/127.0.0.1/tcp/9090".to_string();
    let world = Felt::from_hex_unchecked(
        "0x01e89f62b131b603182587f456573804202bb4db5223bd3bf8d1846b51c47e60",
    );

    let res = Client::new(torii_url, rpc_url, relay_url, world).await;
    match res {
        Ok(client) => {
            let _ = tx.send(client);
            println!("successfully connected to torii!")
        }
        Err(e) => println!("{:?}", e),
    }
}

fn query_all_entities(rt: Res<TokioRuntimeResource>, client: Res<ConnectedClient>) {
    let query = ToriiQuery {
        clause: None,
        limit: 100,
        offset: 0,
        dont_include_hashed_keys: true,
        order_by: vec![],
        entity_models: vec![],
        entity_updated_after: 0,
    };

    let client = client.0.clone();
    rt.0.spawn(async move {
        let res = client.entities(query).await;
        println!("{res:?}");
    });
}
