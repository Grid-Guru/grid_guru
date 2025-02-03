// define the interface
#[starknet::interface]
trait IActions<TState> {
    fn create_game(ref self: TState);
}

// dojo decorator
#[dojo::contract]
pub mod actions {
    use super::{IActions};

    use grid_guru::components::playable::PlayableComponent;
    use grid_guru::constants::DEFAULT_NS;

    use dojo::world::WorldStorage;

    component!(path: PlayableComponent, storage: playable, event: PlayableEvent);
    impl PlayableInternalImpl = PlayableComponent::InternalImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        playable: PlayableComponent::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        PlayableEvent: PlayableComponent::Event,
    }

    fn dojo_init(self: @ContractState) {}

    #[abi(embed_v0)]
    impl ActionsImpl of IActions<ContractState> {
        fn create_game(ref self: ContractState) {
            let world = self.world_storage();
            self.playable.create_game(world);
        }
    }

    #[generate_trait]
    impl Private of PrivateTrait {
        /// Use the default namespace "dojo_starter". This function is handy since the ByteArray
        /// can't be const.
        fn world_storage(self: @ContractState) -> WorldStorage {
            self.world(@DEFAULT_NS())
        }
    }
}
