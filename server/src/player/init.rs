use crate::entity::{
    degrees_to_stops, LastKnownPositionComponent, PacketCreatorComponent, PlayerComponent,
    VelocityComponent,
};
use crate::entity::{Metadata, NamedComponent, PositionComponent};
use crate::network::PlayerPreJoinEvent;
use crate::player::{ChunkPendingComponent, InventoryComponent, LoadedChunksComponent};
use crate::prelude::*;
use feather_core::level::LevelData;
use feather_core::packet::SpawnPlayer;
use feather_core::Position;
use feather_core::{Gamemode, Packet};
use hashbrown::HashSet;
use shrev::{EventChannel, ReaderId};
use specs::{Entity, SystemData, WorldExt};
use specs::{Read, System, World, WriteStorage};
use std::path::Path;
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
        WriteStorage<'a, VelocityComponent>,
        WriteStorage<'a, NamedComponent>,
        WriteStorage<'a, ChunkPendingComponent>,
        WriteStorage<'a, LoadedChunksComponent>,
        WriteStorage<'a, InventoryComponent>,
        WriteStorage<'a, Metadata>,
        WriteStorage<'a, LastKnownPositionComponent>,
        WriteStorage<'a, PacketCreatorComponent>,
        Read<'a, LevelData>,
        Read<'a, Arc<Config>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            join_events,
            mut player_comps,
            mut positions,
            mut velocities,
            mut nameds,
            mut chunk_pending_comps,
            mut loaded_chunk_comps,
            mut inventory_comps,
            mut metadata,
            mut last_positions,
            mut packet_creators,
            level,
            config,
        ) = data;

        // Run through events
        for event in join_events.read(&mut self.join_event_reader.as_mut().unwrap()) {
            // Load player data
            let uuid = event.uuid;
            // If this is a new player, set gamemode to server's default (config)
            let default_gamemode = &config.server.default_gamemode.clone();
            let world_dir = Path::new(&config.world.name);

            debug!("Loading player data for UUID {}", uuid);
            let (gamemode, pos, velocity, inventory_slots) =
                match feather_core::player_data::load_player_data(world_dir, uuid) {
                    Ok(data) => (
                        Gamemode::from_id(data.gamemode as u8),
                        data.entity.read_position(),
                        data.entity.read_velocity(),
                        data.inventory,
                    ),
                    Err(_) => (
                        Gamemode::from_string(default_gamemode.as_str()),
                        None, // Invalid position will default to world spawn
                        None,
                        vec![], // Empty inventory
                    ),
                };

            let player_comp = PlayerComponent {
                profile_properties: event.profile_properties.clone(),
                gamemode,
            };
            player_comps.insert(event.player, player_comp).unwrap();

            let spawn_pos = pos.unwrap_or(position!(
                f64::from(level.spawn_x),
                f64::from(level.spawn_y),
                f64::from(level.spawn_z)
            ));
            let position = PositionComponent {
                current: spawn_pos,
                previous: spawn_pos,
            };
            positions.insert(event.player, position).unwrap();

            let velocity = VelocityComponent(velocity.unwrap_or_else(|| glm::vec3(0.0, 0.0, 0.0)));
            velocities.insert(event.player, velocity).unwrap();

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

            let mut inventory_comp = InventoryComponent::new();
            for slot in inventory_slots {
                let slot_index = slot.convert_index();
                if let Some(slot_index) = slot_index {
                    inventory_comp.set_item_at(slot_index, slot.to_stack());
                }
            }
            inventory_comps
                .insert(event.player, inventory_comp)
                .unwrap();

            let last_position = LastKnownPositionComponent::default();
            last_positions.insert(event.player, last_position).unwrap();

            let meta = Metadata::Player(crate::entity::metadata::Player::default());
            metadata.insert(event.player, meta).unwrap();

            let packet_creator = PacketCreatorComponent(&create_packet);
            packet_creators
                .insert(event.player, packet_creator)
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

fn create_packet(world: &World, entity: Entity) -> Box<dyn Packet> {
    let positions = world.read_component::<PositionComponent>();
    let nameds = world.read_component::<NamedComponent>();
    let metas = world.read_component::<Metadata>();

    let position = positions.get(entity).unwrap();

    let packet = SpawnPlayer {
        entity_id: entity.id() as i32,
        player_uuid: nameds.get(entity).unwrap().uuid,
        x: position.current.x,
        y: position.current.y,
        z: position.current.z,
        yaw: degrees_to_stops(position.current.yaw),
        pitch: degrees_to_stops(position.current.pitch),
        metadata: metas.get(entity).unwrap().to_full_raw_metadata(),
    };

    Box::new(packet)
}
