use crate::plugins::tokio::{TokioRuntimeResource, TokioRuntimeState};
use bevy::prelude::*;
use bevy::tasks::futures_lite::StreamExt;
use dojo_types::schema::Struct as DojoStruct;
use starknet::core::types::Felt;
use tokio::sync::mpsc;
use torii_client::client::error::Error as ToriiError;
use torii_client::client::Client as ToriiClient;
use torii_grpc::types::schema::Entity as ToriiEntity;
use torii_grpc::types::Query as ToriiQuery;

pub struct ToriiPlugin;
impl Plugin for ToriiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_entity_channel_resource);
        app.add_systems(
            OnEnter(TokioRuntimeState::Ready),
            spawn_tokio_runtime_thread,
        );
        app.add_systems(Update, spawn_torii_entities);
    }
}

#[derive(Resource)]
pub struct ToriiChannel {
    rx: mpsc::Receiver<ToriiEntity>,
    tx: mpsc::Sender<ToriiEntity>,
}

#[derive(Reflect, Debug, PartialEq, Eq, Clone)]
pub struct BevyFelt {
    felt_string: String,
}
impl From<Felt> for BevyFelt {
    fn from(value: Felt) -> Self {
        BevyFelt {
            felt_string: value.to_string(),
        }
    }
}

#[derive(Component, Debug)]
pub struct BevyfiedDojoEntity {
    pub keys: BevyFelt,
    pub models: Vec<DojoStruct>,
}

#[derive(Event, Debug)]
pub struct UpdatedBevyfiedDojoEntity;

fn spawn_torii_entities(
    mut commands: Commands,
    mut channel: ResMut<ToriiChannel>,
    mut query: Query<&mut BevyfiedDojoEntity>,
) {
    if let Ok(entity_from_torii) = channel.rx.try_recv() {
        if let Some(mut existing_entity) = query
            .iter_mut()
            .find(|e| e.keys.felt_string == entity_from_torii.hashed_keys.to_string())
        {
            info!("updating existing bevyfied entity...");
            info!("from: {existing_entity:?}");
            existing_entity.models = entity_from_torii.models.clone();
            info!("to: {existing_entity:?}");
        } else {
            let new_entity = BevyfiedDojoEntity {
                keys: BevyFelt {
                    felt_string: entity_from_torii.hashed_keys.to_string(),
                },
                models: entity_from_torii.models,
            };
            info!("created new bevyfied dojo entity: {new_entity:?}");
            commands.spawn(new_entity);
        }

        commands.trigger(UpdatedBevyfiedDojoEntity);
    }
}

fn setup_entity_channel_resource(mut commands: Commands) {
    let (tx, rx) = mpsc::channel::<ToriiEntity>(64);
    commands.insert_resource(ToriiChannel { rx, tx });
}

fn spawn_tokio_runtime_thread(rt: Res<TokioRuntimeResource>, channel: Res<ToriiChannel>) {
    let tx = channel.tx.clone();
    let _ = rt.0.spawn(async move {
        if let Ok(client) = create_torii_client().await {
            if let Ok(list_of_existing_entities) = sync_entities(&client).await {
                for entity in list_of_existing_entities.iter() {
                    info!("torii sync: {entity:?}");
                    let _res = tx.clone().send(entity.clone()).await;
                }
            }

            if let Ok(mut stream) = stream_entities(&client).await {
                loop {
                    match stream.try_next().await {
                        Ok(item) => {
                            if let Some((_, entity)) = item {
                                info!("torii stream: {entity:?}");
                                let _res = tx.clone().send(entity.clone()).await;
                            }
                        }
                        Err(e) => error!("{e}"),
                    }
                }
            }
        }
    });
}

async fn create_torii_client() -> Result<ToriiClient, ToriiError> {
    let torii_url = "http://localhost:8080".to_string();
    let rpc_url = "http://127.0.0.1:5050".to_string();
    let relay_url = "/ip4/127.0.0.1/tcp/9090".to_string();
    let world = Felt::from_hex_unchecked(
        "0x057a3f7a51ea6dd81fc1362300aaf3cfbcd84fedf1016a5f43a0694cffea39c",
    );

    ToriiClient::new(torii_url, rpc_url, relay_url, world).await
}

async fn sync_entities(client: &ToriiClient) -> Result<Vec<ToriiEntity>, ToriiError> {
    let query = ToriiQuery {
        clause: None,
        limit: 100,
        offset: 0,
        dont_include_hashed_keys: false,
        order_by: vec![],
        entity_models: vec![],
        entity_updated_after: 0,
    };

    client.entities(query).await
}

async fn stream_entities(
    client: &ToriiClient,
) -> Result<torii_grpc::client::EntityUpdateStreaming, ToriiError> {
    client.on_entity_updated(vec![]).await
}
