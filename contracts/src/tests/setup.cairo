mod setup {
    // Core imports
    use core::debug::PrintTrait;

    // Starknet imports
    use starknet::ContractAddress;
    use starknet::testing::{set_contract_address, set_caller_address};

    // Dojo imports
    use dojo::model::{ModelStorage, ModelValueStorage, ModelStorageTest};
    use dojo::world::{WorldStorage, WorldStorageTrait};
    use dojo_cairo_test::{
        spawn_test_world, NamespaceDef, ContractDef, TestResource, ContractDefTrait,
        WorldStorageTestTrait,
    };

    // Internal imports
    use grid_guru::models::{index as models};
    use grid_guru::systems::actions::{
        actions, IActions, IActionsDispatcher, IActionsDispatcherTrait,
    };
    use grid_guru::constants::DEFAULT_NS;

    #[starknet::interface]
    trait IDojoInit<ContractState> {}

    #[derive(Drop)]
    struct Systems {
        actions: IActionsDispatcher,
    }

    #[inline]
    fn setup_namespace() -> NamespaceDef {
        NamespaceDef {
            namespace: DEFAULT_NS(),
            resources: [
                TestResource::Model(models::m_Player::TEST_CLASS_HASH),
                TestResource::Model(models::m_Game::TEST_CLASS_HASH),
                TestResource::Model(models::m_Position::TEST_CLASS_HASH),
                TestResource::Contract(actions::TEST_CLASS_HASH),
            ]
                .span(),
        }
    }

    fn setup_contracts() -> Span<ContractDef> {
        [
            ContractDefTrait::new(@DEFAULT_NS(), @"actions")
                .with_writer_of([dojo::utils::bytearray_hash(@DEFAULT_NS())].span()),
        ]
            .span()
    }

    #[inline(always)]
    fn spawn_game() -> (WorldStorage, Systems) {
        // [Setup] World
        let namespace_def = setup_namespace();
        let world = spawn_test_world([namespace_def].span());
        world.sync_perms_and_inits(setup_contracts());

        // [Setup] Systems
        let (actions_address, _) = world.dns(@"actions").unwrap();
        let systems = Systems { actions: IActionsDispatcher { contract_address: actions_address } };

        // [Return]
        (world, systems)
    }
}
