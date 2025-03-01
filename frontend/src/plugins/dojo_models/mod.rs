use bevy::prelude::*;
use dojo_types::schema::Struct as DojoStruct;

use crate::plugins::networking::torii::{BevyFelt, BevyfiedDojoEntity, UpdatedBevyfiedDojoEntity};

pub struct DojoModelsPlugin;
impl Plugin for DojoModelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(convert_to_bevy);
        app.add_observer(list_entities);
        app.register_type::<DojoKey>();
        app.register_type::<Game>();
        app.register_type::<Player>();
        app.register_type::<Tile>();
        app.register_type::<u128>();
    }
}

fn list_entities(
    _trigger: Trigger<Converted>,
    mut commands: Commands,
    query: Query<(Entity, &DojoKey)>,
) {
    let count = query.iter().count();
    info!("number of converted dojo entities: {count:?}");
    for (id, _) in query.iter() {
        commands.entity(id).log_components();
    }
}

#[derive(Event, Debug)]
struct Converted;

fn convert_to_bevy(
    _trigger: Trigger<UpdatedBevyfiedDojoEntity>,
    mut commands: Commands,
    query_bevyfied: Query<&BevyfiedDojoEntity>,
    query_dojokey: Query<(Entity, &DojoKey)>,
) {
    for bevyfied_entity in query_bevyfied.iter() {
        let key = bevyfied_entity.keys.clone();
        let bevy_id;
        if let Some((id, _)) = query_dojokey.iter().find(|(_, k)| k.0 == key) {
            bevy_id = id;
        } else {
            let id = commands.spawn(DojoKey(key)).id();
            bevy_id = id;
        }

        let dojo_models = bevyfied_entity.models.clone();
        for dojo_model in dojo_models.iter() {
            let struct_name = dojo_model.name.as_str();
            match struct_name {
                "grid_guru-Player" => {
                    let player = Player::from(dojo_model.clone());
                    let or_player = player.clone();
                    info!("created bevy native player component: {player:?}");
                    commands
                        .entity(bevy_id)
                        .entry::<Player>()
                        .and_modify(move |mut p| {
                            p.game_id = player.game_id;
                            p.address = player.address;
                            p.score = player.score;
                        })
                        .or_insert(or_player.clone());
                }
                "grid_guru-Tile" => {
                    let tile = Tile::from(dojo_model.clone());
                    let or_tile = tile.clone();
                    info!("created bevy native tile component: {tile:?}");
                    commands
                        .entity(bevy_id)
                        .entry::<Tile>()
                        .and_modify(move |mut t| {
                            t.game_id = tile.game_id;
                            t.x = tile.x;
                            t.y = tile.y;
                            t.owner = tile.owner;
                        })
                        .or_insert(or_tile);
                }
                "grid_guru-Game" => {
                    let game = Game::from(dojo_model.clone());
                    let or_game = game.clone();
                    info!("created bevy native game component: {game:?}");
                    commands
                        .entity(bevy_id)
                        .entry::<Game>()
                        .and_modify(move |mut g| {
                            g.game_id = game.game_id;
                            g.player_one = game.player_one;
                            g.player_two = game.player_two;
                            g.current_player = game.current_player;
                            g.winner = game.winner;
                            g.move_count = game.move_count;
                            g.status = game.status;
                        })
                        .or_insert(or_game);
                }
                _ => {}
            }
        }
    }
    commands.trigger(Converted);
}

#[derive(Component, Debug, Reflect)]
pub struct DojoKey(BevyFelt);

#[derive(Component, Debug, Reflect, Clone)]
pub struct Game {
    pub game_id: u128,
    pub player_one: BevyFelt,
    pub player_two: BevyFelt,
    pub current_player: BevyFelt,
    pub winner: BevyFelt,
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
            .unwrap()
            .into();
        let player_two = value.children[2]
            .ty
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap()
            .into();
        let current_player = value.children[3]
            .ty
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap()
            .into();
        let winner = value.children[4]
            .ty
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap()
            .into();
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

#[derive(Debug, Clone, Copy, Reflect)]
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

#[derive(Component, Debug, Reflect, Clone)]
pub struct Player {
    pub game_id: u128,
    pub address: BevyFelt,
    pub score: u8,
    pub grid: BevyFelt,
    pub x: u8,
    pub y: u8,
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
            .unwrap()
            .into();
        let score = value.children[2]
            .ty
            .as_primitive()
            .unwrap()
            .as_u8()
            .unwrap();
        let grid = value.children[3]
            .ty
            .as_primitive()
            .unwrap()
            .as_felt252()
            .unwrap()
            .into();
        let x = value.children[4]
            .ty
            .as_primitive()
            .unwrap()
            .as_u8()
            .unwrap();
        let y = value.children[5]
            .ty
            .as_primitive()
            .unwrap()
            .as_u8()
            .unwrap();

        Player {
            game_id,
            address,
            score,
            grid,
            x,
            y,
        }
    }
}

#[derive(Component, Debug, Reflect, Clone)]
pub struct Tile {
    pub game_id: u128,
    pub x: u8,
    pub y: u8,
    pub owner: BevyFelt,
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
            .unwrap()
            .into();

        Tile {
            game_id,
            x,
            y,
            owner,
        }
    }
}
