#[starknet::component]
pub mod PlayableComponent {
    // Dojo imports
    use dojo::world::WorldStorage;
    use dojo::world::{IWorldDispatcherTrait};

    // Starknet imports
    use starknet::{get_caller_address};

    // Internal imports
    use grid_guru::store::{Store, StoreTrait};
    use grid_guru::models::player::{Player, PlayerTrait};
    use grid_guru::models::game::{Game, GameTrait};
    //use grid_guru::models::tile::{Tile, TileTrait};

    // Errors
    pub mod errors {}

    // Storage
    #[storage]
    pub struct Storage {}

    // Events
    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {}

    #[generate_trait]
    pub impl InternalImpl<TState, +HasComponent<TState>> of InternalTrait<TState> {
        fn create_game(ref self: ComponentState<TState>, world: WorldStorage) {
            let mut store: Store = StoreTrait::new(world);
            let game_id: u128 = world.dispatcher.uuid().into() + 1;

            let mut player: Player = PlayerTrait::new(game_id, get_caller_address());
            let mut game: Game = GameTrait::new(game_id, player.address);

            store.set_player(player);
            store.set_game(game);
        }
    }
}
