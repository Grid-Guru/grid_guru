pub use grid_guru::models::index::Player;

use origami_map::helpers::bitmap::Bitmap;
use origami_map::map::{Map, MapTrait};
use starknet::ContractAddress;

pub mod errors {
    pub const CLAIMED_TILE: felt252 = 'Claimed tile';
    pub const OPPONENT_TILE: felt252 = 'Tile claimed by opponent';
    pub const NO_PATH_TO_TARGET: felt252 = 'No path to target';
    pub const X_OUT_BOUNDS: felt252 = 'x out of bounds';
    pub const Y_OUT_BOUNDS: felt252 = 'y out of bounds';
}

#[generate_trait]
pub impl PlayerImpl of PlayerTrait {
    #[inline]
    fn new(game_id: u128, address: ContractAddress, grid: felt252, x: u8, y: u8) -> Player {
        Player { game_id, address, score: 0, grid, x, y }
    }

    fn move(ref self: Player, opponent_grid: felt252, x: u8, y: u8) {
        // check if the move is valid
        assert(x < 8, errors::X_OUT_BOUNDS);
        assert(y < 8, errors::Y_OUT_BOUNDS);

        let index = ((7 - x) + ((7 - y) * 8));
        assert(Bitmap::get(self.grid, index) == 0, errors::CLAIMED_TILE);
        assert(Bitmap::get(opponent_grid, index) == 0, errors::OPPONENT_TILE);

        // temporarily set the tile in 1
        let tmp_grid = Bitmap::set(self.grid, index);

        // check if there is a path to target
        let origami_map = MapTrait::new(tmp_grid, 8, 8, 'SEED');
        let current_pos = ((7 - self.x) + ((7 - self.y) * 8));
        let path_to_target = origami_map.search_path(current_pos, index);
        assert(path_to_target.len() > 0, errors::NO_PATH_TO_TARGET);
        
        // set the tile in 1 and move player
        self.grid = tmp_grid;
        self.x = x;
        self.y = y;
    }

    fn path_to_tile(ref self: Player, idx: u8) -> bool {
        let x = 7 - (idx % 8);
        let y = 7 - (idx / 8);

        let result = if (x + 1) <= 7 && Bitmap::get(self.grid, idx - 1) == 1 {
            true
        } else if (x > 0) && (x - 1) >= 0 && Bitmap::get(self.grid, idx + 1) == 1 {
            true
        } else if (y + 1) <= 7 && Bitmap::get(self.grid, idx - 8) == 1 {
            true
        } else if (y > 0) && (y - 1) >= 0 && Bitmap::get(self.grid, idx + 8) == 1 {
            true
        } else {
            false
        };
        result
    }

    fn remaining_moves(ref self: Player, opponent_grid: felt252) -> Span<u8> {
        let mut idx = 0;
        let mut result = array![];
        loop{
            if idx == 64 {
                break;
            }
            if Bitmap::get(self.grid, idx) == 0 {
                if Bitmap::get(opponent_grid, idx) == 0 && self.path_to_tile(idx) {
                    result.append(idx);
                }
            }
            idx += 1;
        };
        result.span()
    }
}

#[generate_trait]
impl PlayerAssert of AssertTrait {}
