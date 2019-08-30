use crate::entity::{EntityType, PlayerComponent};
use crate::entity::{Metadata, NamedComponent, PositionComponent};
use crate::network::PlayerPreJoinEvent;
use crate::player::{ChunkPendingComponent, InventoryComponent, LoadedChunksComponent};
use crate::prelude::*;
use feather_core::level::LevelData;
use feather_core::player_data::UNSET_GAMEMODE;
use feather_core::Gamemode;
use feather_core::Position;
use hashbrown::HashSet;
use shrev::{EventChannel, ReaderId};
use specs::SystemData;
use specs::{Read, System, World, WriteStorage};
use std::sync::Arc;

/// System for initializing the necessary components
/// when a player joins.
#[derive(Default)]
pub struct PlayerInitSystem {
    join_event_reader: Option<ReaderId<PlayerPreJoinEvent>>,
}

impl<'a> System<'a> for PlayerInitSystem {
    type SystemData = (
        Read<'a, EventChannel<PlayerPreJoinEvent>>,
        WriteStorage<'a, PlayerComponent>,
        WriteStorage<'a, PositionComponent>,
        WriteStorage<'a, NamedComponent>,
        WriteStorage<'a, ChunkPendingComponent>,
        WriteStorage<'a, LoadedChunksComponent>,
        WriteStorage<'a, InventoryComponent>,
        WriteStorage<'a, EntityType>,
        WriteStorage<'a, Metadata>,
        Read<'a, LevelData>,
        Read<'a, Arc<Config>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            events,
            mut player_comps,
            mut positions,
            mut nameds,
            mut chunk_pending_comps,
            mut loaded_chunk_comps,
            mut inventory_comps,
            mut entity_types,
            mut metadata,
            level,
            config,
        ) = data;

        // Run through events
        for event in events.read(&mut self.join_event_reader.as_mut().unwrap()) {
            // Load player data
            let uuid = event.uuid;
            debug!("Loading player data for UUID {}", uuid);

            let player_data = feather_core::player_data::load_player_data(uuid).unwrap_or_default();

            // If this is a new player, set gamemode to server's default (config)
            let default_gamemode = &config.server.default_gamemode;
            let gamemode = match player_data.gamemode {
                UNSET_GAMEMODE => Gamemode::from_string(default_gamemode.to_string()),
                _ => Gamemode::from_id(player_data.gamemode as u8),
            };

            let player_comp = PlayerComponent {
                profile_properties: event.profile_properties.clone(),
                gamemode,
            };
            player_comps.insert(event.player, player_comp).unwrap();

            let spawn_pos = position!(
                f64::from(level.spawn_x),
                f64::from(level.spawn_y),
                f64::from(level.spawn_z)
            );

            let position = PositionComponent {
                current: spawn_pos,
                previous: spawn_pos,
            };
            positions.insert(event.player, position).unwrap();

            let named = NamedComponent {
                display_name: event.username.clone(),
                uuid: event.uuid,
            };
            nameds.insert(event.player, named).unwrap();

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

            let ty = EntityType::Player;
            entity_types.insert(event.player, ty).unwrap();

            let meta = Metadata::Player(crate::entity::metadata::Player::default());
            metadata.insert(event.player, meta).unwrap();
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
