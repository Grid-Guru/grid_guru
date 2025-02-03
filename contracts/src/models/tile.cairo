use starknet::ContractAddress;
pub use grid_guru::models::index::Tile;
//use dojo::world::WorldStorage;
//use grid_guru::store::{Store, StoreTrait};

pub mod errors {}

#[generate_trait]
pub impl TileImpl of TileTrait {
    #[inline]
    fn new(game_id: u128, x: u8, y: u8, owner: ContractAddress) -> Tile {
        Tile { game_id, x, y, owner }
    }

    #[inline]
    fn claim(ref self: Tile, player: ContractAddress) {
        self.owner = player;
    }

    #[inline]
    fn is_valid_move(
        ref self: Tile, ref tiles: Array<Tile>, x: u8, y: u8, player: ContractAddress,
    ) -> bool {
        let tiles_snapshot = @tiles;

        if x >= 8 || y >= 8 {
            return false;
        }

        if self.is_tile_occupied(tiles_snapshot, x, y) {
            return false;
        }

        self.has_adjacent_tile(tiles_snapshot, x, y, player)
    }

    #[inline]
    fn is_tile_occupied(ref self: Tile, tiles: @Array<Tile>, x: u8, y: u8) -> bool {
        let mut i = 0;
        loop {
            if i >= tiles.len() {
                break false;
            }
            let tile = *tiles[i];
            if tile.x == x && tile.y == y {
                break true;
            }
            i += 1;
        }
    }

    #[inline]
    fn has_adjacent_tile(
        ref self: Tile, tiles: @Array<Tile>, x: u8, y: u8, player: ContractAddress,
    ) -> bool {
        let adjacents = self.get_adjacent_positions(x, y);

        let mut i = 0;
        loop {
            if i >= tiles.len() {
                break false;
            }
            let tile = *tiles[i];
            if tile.owner == player {
                let mut j = 0;
                let is_adjacent = loop {
                    if j >= adjacents.len() {
                        break false;
                    }
                    let (adj_x, adj_y) = *adjacents[j];
                    if tile.x == adj_x && tile.y == adj_y {
                        break true;
                    }
                    j += 1;
                };
                if is_adjacent {
                    break true;
                }
            }
            i += 1;
        }
    }

    #[inline]
    fn get_adjacent_positions(ref self: Tile, x: u8, y: u8) -> Array<(u8, u8)> {
        let mut positions = ArrayTrait::new();

        if x > 0 {
            positions.append((x - 1, y));
        }

        if x < 7 {
            positions.append((x + 1, y));
        }

        if y > 0 {
            positions.append((x, y - 1));
        }

        if y < 7 {
            positions.append((x, y + 1));
        }

        positions
    }
}

#[generate_trait]
pub impl TileUtils of TileUtilsTrait {}

#[generate_trait]
impl TileAssert of AssertTrait {}
