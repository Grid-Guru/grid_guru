use bevy::{input::keyboard::Key, prelude::*};
use dojo_types::schema::Struct as DojoStruct;
use starknet::core::types::Felt;

use super::torii::BevyfiedDojoEntity;

pub struct DojoModelsPlugin;
impl Plugin for DojoModelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, convert_to_bevy);
    }
}

fn convert_to_bevy(mut commands: Commands, query: Query<&BevyfiedDojoEntity>) {
    for dojo_entity in query.iter() {
        let dojo_key = dojo_entity.keys;
        let id = commands.spawn(DojoKey(dojo_key)).id();

        let dojo_models = dojo_entity.models.clone();
        for dojo_model in dojo_models.iter() {
            let struct_name = dojo_model.name.as_str();
            match struct_name {
                "grid_guru-Player" => {
                    let player = Player::from(dojo_model.clone());
                    info!("created bevy native player component: {player:?}");
                    commands.entity(id).insert(player);
                }
                "grid_guru-Tile" => {
                    let tile = Tile::from(dojo_model.clone());
                    info!("created bevy native tile component: {tile:?}");
                    commands.entity(id).insert(tile);
                }
                "grid_guru-Game" => {
                    let game = Game::from(dojo_model.clone());
                    info!("created bevy native game component: {game:?}");
                    commands.entity(id).insert(game);
                }
                _ => {}
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct DojoKey(Felt);

#[derive(Component, Debug)]
pub struct Game {
    pub game_id: u128,
    pub player_one: Felt,
    pub player_two: Felt,
    pub current_player: Felt,
    pub winner: Felt,
    pub move_count: u8,
    pub status: GameStatus,
}
impl From<DojoStruct> for Game {
    fn from(value: DojoStruct) -> Self {
        let game_id = value.children[0]
            .ty
            .as_primitive()
            .unwrap()
            .as_u128()
            .unwrap();
        let player_one = value.children[1]
            .ty
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap();
        let player_two = value.children[2]
            .ty
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap();
        let current_player = value.children[3]
            .ty
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap();
        let winner = value.children[4]
            .ty
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap();
        let move_count = value.children[5]
            .ty
            .as_primitive()
            .unwrap()
            .as_u8()
            .unwrap();
        let status_id = value.children[6].ty.as_enum().unwrap().option.unwrap();
        let status = GameStatus::from(status_id);

        Game {
            game_id,
            player_one,
            player_two,
            current_player,
            winner,
            move_count,
            status,
        }
    }
}

#[derive(Debug)]
pub enum GameStatus {
    Pending,
    InProgress,
    Completed,
    Draw,
    Abandoned,
    TimedOut,
}
impl From<u8> for GameStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => GameStatus::Pending,
            1 => GameStatus::InProgress,
            2 => GameStatus::Completed,
            3 => GameStatus::Draw,
            4 => GameStatus::Abandoned,
            _ => GameStatus::TimedOut,
        }
    }
}

#[derive(Component, Debug)]
pub struct Player {
    pub game_id: u128,
    pub address: Felt,
    pub score: u8,
}
impl From<DojoStruct> for Player {
    fn from(value: DojoStruct) -> Self {
        let game_id = value.children[0]
            .ty
            .as_primitive()
            .unwrap()
            .as_u128()
            .unwrap();
        let address = value.children[1]
            .ty
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap();
        let score = value.children[2]
            .ty
            .as_primitive()
            .unwrap()
            .as_u8()
            .unwrap();

        Player {
            game_id,
            address,
            score,
        }
    }
}

// Struct { name: "grid_guru-Player", children: [Member { name: "game_id", ty: Primitive(U128(Some(3))), key: true }, Member { name: "address", ty: Primitive(ContractAddress(Some(0x127fd5f1fe78a71f8bcd1fec63e3fe2f0486b6ecd5c86a0466c3a21fa5cfcec))), key: true }, Member { name: "score", ty: Primitive(U8(Some(0))), key: false }] }
#[derive(Component, Debug)]
pub struct Tile {
    pub game_id: u128,
    pub x: u8,
    pub y: u8,
    pub owner: Felt,
}
impl From<DojoStruct> for Tile {
    fn from(value: DojoStruct) -> Self {
        let game_id = value.children[0]
            .ty
            .as_primitive()
            .unwrap()
            .as_u128()
            .unwrap();
        let x = value.children[1]
            .ty
            .as_primitive()
            .unwrap()
            .as_u8()
            .unwrap();
        let y = value.children[2]
            .ty
            .as_primitive()
            .unwrap()
            .as_u8()
            .unwrap();
        let owner = value.children[3]
            .ty
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap();

        Tile {
            game_id,
            x,
            y,
            owner,
        }
    }
}
