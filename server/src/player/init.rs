use crate::entity::{EntityComponent, PlayerComponent};
use crate::joinhandler::SPAWN_POSITION;
use crate::network::PlayerPreJoinEvent;
use crate::player::{ChunkPendingComponent, InventoryComponent, LoadedChunksComponent};
use feather_core::Gamemode;
use hashbrown::HashSet;
use shrev::{EventChannel, ReaderId};
use specs::SystemData;
use specs::{Read, System, World, WriteStorage};

/// System for initializing the necessary components
/// when a player joins.
#[derive(Default)]
pub struct PlayerInitSystem {
    join_event_reader: Option<ReaderId<PlayerPreJoinEvent>>,
}

impl PlayerInitSystem {
    pub fn new() -> Self {
        Self {
            join_event_reader: None,
        }
    }
}

impl<'a> System<'a> for PlayerInitSystem {
    type SystemData = (
        Read<'a, EventChannel<PlayerPreJoinEvent>>,
        WriteStorage<'a, PlayerComponent>,
        WriteStorage<'a, EntityComponent>,
        WriteStorage<'a, ChunkPendingComponent>,
        WriteStorage<'a, LoadedChunksComponent>,
        WriteStorage<'a, InventoryComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            events,
            mut player_comps,
            mut entity_comps,
            mut chunk_pending_comps,
            mut loaded_chunk_comps,
            mut inventory_comps,
        ) = data;

        // Run through events
        for event in events.read(&mut self.join_event_reader.as_mut().unwrap()) {
            let player_comp = PlayerComponent {
                profile_properties: event.profile_properties.clone(),
                gamemode: Gamemode::Creative,
            };
            player_comps.insert(event.player, player_comp).unwrap();

            let entity_comp = EntityComponent {
                uuid: event.uuid,
                display_name: event.username.clone(),
                position: SPAWN_POSITION,
                on_ground: true,
            };
            entity_comps.insert(event.player, entity_comp).unwrap();

            let chunk_pending_comp = ChunkPendingComponent {
                pending: HashSet::new(),
            };
            chunk_pending_comps
                .insert(event.player, chunk_pending_comp)
                .unwrap();

            let loaded_chunk_comp = LoadedChunksComponent::default();
            loaded_chunk_comps
                .insert(event.player, loaded_chunk_comp)
                .unwrap();

            let inventory_comp = InventoryComponent::new();
            inventory_comps
                .insert(event.player, inventory_comp)
                .unwrap();
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.join_event_reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerPreJoinEvent>>()
                .register_reader(),
        );
    }
}
