use ahash::AHashSet;
use common::entities::player::HotbarSlot;
use either::Either;
use flume::{Receiver, Sender};
use itertools::Itertools;
use slotmap::SlotMap;
use std::any::type_name;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::{collections::VecDeque, sync::Arc};
use uuid::Uuid;
use vane::Component;

use common::world::Dimensions;
use common::{
    chat::{ChatKind, ChatMessage},
    PlayerWindow,
};
use libcraft::biome::BiomeList;
use libcraft::items::InventorySlot;
use libcraft::{
    Area, BlockState, ChunkPosition, EntityKind, EntityMetadata, Gamemode, Inventory, Particle,
    Position, ProfileProperty, Text, Title, ValidBlockPosition,
};
use packets::server::{SetSlot, SpawnLivingEntity};
use protocol::packets::server::{
    ChangeGameState, ClearTitles, DimensionCodec, DimensionCodecEntry, DimensionCodecRegistry,
    EntityEquipment, EntityPosition, EntityPositionAndRotation, EntityTeleport, EquipmentEntry,
    EquipmentSlot, GameStateChange, HeldItemChange, PlayerAbilities, Respawn, SetTitleSubtitle,
    SetTitleText, SetTitleTimes,
};
use protocol::{
    packets::{
        self,
        server::{
            AddPlayer, Animation, BlockChange, ChatPosition, ChunkData, DestroyEntities,
            Disconnect, EntityAnimation, EntityHeadLook, JoinGame, KeepAlive, PlayerInfo,
            PlayerPositionAndLook, PluginMessage, SendEntityMetadata, SpawnPlayer, UnloadChunk,
            UpdateViewPosition, WindowItems,
        },
    },
    ClientPlayPacket, Nbt, ProtocolVersion, ServerPlayPacket, VarInt, Writeable,
};
use quill::components::{EntityDimension, EntityWorld, OnGround, PreviousGamemode};
use quill::ChunkHandle;

use crate::{
    entities::{PreviousOnGround, PreviousPosition},
    initial_handler::NewPlayer,
    network_id_registry::NetworkId,
    Options,
};

/// Max number of chunks to send to a client per tick.
const MAX_CHUNKS_PER_TICK: usize = 10;

slotmap::new_key_type! {
    pub struct ClientId;
}

impl Component for ClientId {}

/// Stores all `Client`s.
#[derive(Default)]
pub struct Clients {
    map: SlotMap<ClientId, Client>,
}

impl Clients {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, client: Client) -> ClientId {
        self.map.insert(client)
    }

    pub fn remove(&mut self, id: ClientId) -> Option<Client> {
        self.map.remove(id)
    }

    pub fn get(&self, id: ClientId) -> Option<&Client> {
        self.map.get(id)
    }

    pub fn get_mut(&mut self, id: ClientId) -> Option<&mut Client> {
        self.map.get_mut(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &'_ Client> + '_ {
        self.map.iter().map(|(_i, client)| client)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &'_ mut Client> + '_ {
        self.map.iter_mut().map(|(_i, client)| client)
    }
}

/// A client connected to a server.
///
/// This struct provides methods to send packets
/// to the client.
pub struct Client {
    packets_to_send: Sender<ServerPlayPacket>,
    received_packets: Receiver<ClientPlayPacket>,
    options: Arc<Options>,
    username: String,
    profile: Vec<ProfileProperty>,
    uuid: Uuid,

    teleport_id_counter: i32,

    network_id: Option<NetworkId>,
    sent_entities: AHashSet<NetworkId>,

    knows_position: bool,
    known_chunks: AHashSet<ChunkPosition>,

    chunk_send_queue: VecDeque<ChunkData>,

    /// The previous own position sent by the client.
    /// Used to detect when we need to teleport the client.
    client_known_position: Option<Position>,

    disconnected: bool,

    dimension: Option<EntityDimension>,
    world: Option<EntityWorld>,
}

impl Client {
    pub fn new(player: NewPlayer, options: Arc<Options>) -> Self {
        Self {
            packets_to_send: player.packets_to_send,
            received_packets: player.received_packets,
            options,
            username: player.username,
            teleport_id_counter: 0,
            network_id: None,
            profile: player.profile,
            uuid: player.uuid,
            sent_entities: AHashSet::new(),
            knows_position: false,
            known_chunks: AHashSet::new(),
            chunk_send_queue: VecDeque::new(),
            client_known_position: None,
            disconnected: false,
            dimension: None,
            world: None,
        }
    }

    pub fn set_client_known_position(&mut self, pos: Position) {
        self.client_known_position = Some(pos);
    }

    pub fn client_known_position(&self) -> Option<Position> {
        self.client_known_position
    }

    pub fn profile(&self) -> &[ProfileProperty] {
        &self.profile
    }

    pub fn network_id(&self) -> Option<NetworkId> {
        self.network_id
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn received_packets(&self) -> impl Iterator<Item = ClientPlayPacket> + '_ {
        self.received_packets.try_iter()
    }

    pub fn is_disconnected(&self) -> bool {
        self.received_packets.is_disconnected() || self.disconnected
    }

    pub fn known_chunks(&self) -> &AHashSet<ChunkPosition> {
        &self.known_chunks
    }

    pub fn knows_own_position(&self) -> bool {
        self.knows_position
    }

    pub fn tick(&mut self) {
        let num_to_send = MAX_CHUNKS_PER_TICK.min(self.chunk_send_queue.len());
        let packets = self
            .chunk_send_queue
            .drain(..num_to_send)
            .collect::<Vec<_>>();
        for packet in packets {
            log::trace!(
                "Sending chunk at {:?} to {}",
                packet.chunk.as_ref().unwrap_left().read().position(),
                self.username
            );
            self.send_packet(packet);
        }
    }

    /// Returns whether the entity with the given ID
    /// is currently loaded on the client.
    pub fn is_entity_loaded(&self, network_id: NetworkId) -> bool {
        self.sent_entities.contains(&network_id)
    }

    pub fn set_network_id(&mut self, network_id: NetworkId) {
        self.network_id = Some(network_id);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn send_join_game(
        &mut self,
        gamemode: Gamemode,
        previous_gamemode: PreviousGamemode,
        dimensions: &Dimensions,
        biomes: &BiomeList,
        max_players: i32,
        dimension: EntityDimension,
        world: EntityWorld,
    ) {
        log::trace!("Sending Join Game to {}", self.username);

        let dimension = dimensions.get(&*dimension).unwrap_or_else(|| panic!("Tried to spawn {} in dimension `{}` but the dimension doesn't exist! Existing dimensions: {}", self.username, *dimension, dimensions.iter().map(|dim| format!("`{}`", dim.info().r#type)).join(", ")));
        let dimension_codec = DimensionCodec {
            registries: HashMap::from_iter([
                (
                    "minecraft:dimension_type".to_string(),
                    DimensionCodecRegistry::DimensionType(
                        dimensions
                            .iter()
                            .enumerate()
                            .map(|(i, dim)| DimensionCodecEntry {
                                name: dim.info().r#type.to_owned(),
                                id: i as i16,
                                element: dim.info().info.clone(),
                            })
                            .collect(),
                    ),
                ),
                (
                    "minecraft:worldgen/biome".to_string(),
                    DimensionCodecRegistry::WorldgenBiome(
                        biomes
                            .iter()
                            .enumerate()
                            .map(|(i, (name, biome))| DimensionCodecEntry {
                                name: name.to_owned(),
                                id: i as i16,
                                element: biome.info.clone(),
                            })
                            .collect(),
                    ),
                ),
            ]),
        };

        self.dimension = Some(EntityDimension(dimension.info().r#type.clone()));
        self.world = Some(world);

        self.send_packet(JoinGame {
            entity_id: self.network_id.expect("No network id! Use client.set_network_id(NetworkId) before calling this method.").0,
            is_hardcore: false,
            gamemode,
            previous_gamemode,
            dimension_names: dimensions
                .iter().map(|dim| dim.info().r#type.clone()).collect(),
            dimension_codec: Nbt(dimension_codec),
            dimension: Nbt(dimension.info().info.clone()),
            dimension_name: dimension.info().r#type.clone(),
            hashed_seed: 0,
            max_players,
            view_distance: self.options.view_distance as i32,
            simulation_distance: self.options.view_distance as i32,
            reduced_debug_info: false,
            enable_respawn_screen: true,
            is_debug: dimension.is_debug(),
            is_flat: dimension.is_flat(),
        });
    }

    pub fn send_brand(&self) {
        let mut data = Vec::new();
        "Feather"
            .to_owned()
            .write(&mut data, ProtocolVersion::V1_18_1)
            .unwrap();
        self.send_plugin_message("minecraft:brand", data)
    }

    pub fn send_plugin_message(&self, channel: impl Into<String>, data: impl Into<Vec<u8>>) {
        let channel = channel.into();
        log::trace!("Sending plugin message {} to {}", channel, self.username);
        self.send_packet(PluginMessage {
            channel,
            data: data.into(),
        })
    }

    pub fn update_own_position(&mut self, new_position: Position) {
        log::trace!(
            "Updating position of {} to {:?}",
            self.username,
            new_position
        );
        self.teleport_id_counter += 1;
        self.send_packet(PlayerPositionAndLook {
            x: new_position.x,
            y: new_position.y,
            z: new_position.z,
            yaw: new_position.yaw,
            pitch: new_position.pitch,
            flags: 0,
            teleport_id: self.teleport_id_counter,
            dismount_vehicle: false,
        });
        self.knows_position = true;
        self.client_known_position = Some(new_position);
    }

    pub fn move_to_dimension(
        &mut self,
        dimension: EntityDimension,
        dimensions: &Dimensions,
        gamemode: Gamemode,
        previous_gamemode: PreviousGamemode,
        world: EntityWorld,
    ) {
        let dimension = dimensions.get(&*dimension).unwrap_or_else(|| panic!("Tried to move {} to dimension `{}` but the dimension doesn't exist! Existing dimensions: {}", self.username, *dimension, dimensions.iter().map(|dim| format!("`{}`", dim.info().r#type)).join(", ")));
        let dimension_info = dimension.info().info.clone();

        self.dimension = Some(EntityDimension(dimension.info().r#type.clone()));
        self.world = Some(world);

        self.send_packet(Respawn {
            dimension: Nbt(dimension_info),
            dimension_name: dimension.info().r#type.clone(),
            hashed_seed: 0,
            gamemode,
            previous_gamemode,
            is_debug: dimension.is_debug(),
            is_flat: dimension.is_flat(),
            copy_metadata: true,
        });

        self.knows_position = false;
        self.client_known_position = None;
        self.unload_all_entities();
    }

    pub fn update_own_chunk(&self, pos: ChunkPosition) {
        log::trace!("Updating chunk position of {} to {:?}", self.username, pos);
        self.send_packet(UpdateViewPosition {
            chunk_x: pos.x,
            chunk_z: pos.z,
        });
    }

    pub fn send_chunk(&mut self, chunk: &ChunkHandle) {
        self.chunk_send_queue.push_back(ChunkData {
            chunk: Either::Left(Arc::clone(chunk)),
        });
        self.known_chunks.insert(chunk.read().position());
    }

    pub fn overwrite_chunk(&self, chunk: &ChunkHandle) {
        self.send_packet(ChunkData {
            chunk: Either::Left(Arc::clone(chunk)),
        });
    }

    pub fn send_block_change(&self, position: ValidBlockPosition, new_block: BlockState) {
        self.send_packet(BlockChange {
            position,
            block: new_block,
        });
    }

    pub fn unload_chunk(&mut self, pos: ChunkPosition) {
        if self.known_chunks.remove(&pos) {
            log::trace!("Unloading chunk at {:?} on {}", pos, self.username);
            self.send_packet(UnloadChunk {
                chunk_x: pos.x,
                chunk_z: pos.z,
            });
        }
    }

    pub fn add_tablist_player(
        &self,
        uuid: Uuid,
        name: String,
        profile: &[ProfileProperty],
        gamemode: Gamemode,
    ) {
        log::trace!("Sending AddPlayer({}) to {}", name, self.username);
        let action = AddPlayer {
            uuid,
            name,
            properties: profile.to_vec(),
            gamemode,
            ping: 0,
            display_name: None,
        };
        self.send_packet(PlayerInfo::AddPlayers(vec![action]));
    }

    pub fn remove_tablist_player(&self, uuid: Uuid) {
        log::trace!("Sending RemovePlayer({}) to {}", uuid, self.username);
        self.send_packet(PlayerInfo::RemovePlayers(vec![uuid]));
    }

    pub fn change_player_tablist_gamemode(&self, uuid: Uuid, gamemode: Gamemode) {
        self.send_packet(PlayerInfo::UpdateGamemodes(vec![(uuid, gamemode)]));
    }

    pub fn unload_entity(&mut self, id: NetworkId) {
        log::trace!("Unloading {:?} on {}", id, self.username);
        self.sent_entities.remove(&id);
        self.send_packet(DestroyEntities {
            entity_ids: vec![id.0.into()],
        });
    }

    pub fn unload_all_entities(&mut self) {
        self.unload_entities(&self.sent_entities.iter().copied().collect_vec())
    }

    pub fn unload_entities(&mut self, ids: &[NetworkId]) {
        if !ids.is_empty() {
            log::trace!("Unloading {:?} on {}", ids, self.username);
            self.sent_entities.retain(|e| !ids.contains(e));
            self.send_packet(DestroyEntities {
                entity_ids: ids.iter().map(|id| VarInt(id.0)).collect(),
            });
        } else {
            log::trace!("Unloading 0 entities on {}", self.username)
        }
    }

    pub fn send_player(&mut self, network_id: NetworkId, uuid: Uuid, pos: Position) {
        log::trace!("Sending {:?} to {}", uuid, self.username);
        assert!(!self.sent_entities.contains(&network_id));
        self.send_packet(SpawnPlayer {
            entity_id: network_id.0,
            player_uuid: uuid,
            x: pos.x,
            y: pos.y,
            z: pos.z,
            yaw: pos.yaw,
            pitch: pos.pitch,
        });
        self.register_entity(network_id);
    }

    pub fn send_living_entity(
        &self,
        network_id: NetworkId,
        uuid: Uuid,
        pos: Position,
        kind: EntityKind,
    ) {
        log::trace!(
            "Spawning a {:?} on {} (entity type ID: {})",
            kind,
            self.username,
            kind.id()
        );
        self.send_packet(SpawnLivingEntity {
            entity_id: network_id.0,
            entity_uuid: uuid,
            kind: kind.id() as i32,
            x: pos.x,
            y: pos.y,
            z: pos.z,
            yaw: pos.yaw,
            pitch: pos.pitch,
            head_pitch: pos.pitch,
            velocity_x: 0,
            velocity_y: 0,
            velocity_z: 0,
        });
    }

    pub fn send_entity_equipment(
        &mut self,
        network_id: NetworkId,
        inventory: &Inventory,
        hotbar_slot: &HotbarSlot,
    ) {
        if Some(network_id) == self.network_id {
            return;
        }

        let entries = vec![
            equipment_entry(EquipmentSlot::Boots, inventory, Area::Boots, 0),
            equipment_entry(EquipmentSlot::Leggings, inventory, Area::Leggings, 0),
            equipment_entry(EquipmentSlot::Chestplate, inventory, Area::Chestplate, 0),
            equipment_entry(EquipmentSlot::Helmet, inventory, Area::Helmet, 0),
            equipment_entry(
                EquipmentSlot::MainHand,
                inventory,
                Area::Hotbar,
                hotbar_slot.get(),
            ),
            equipment_entry(EquipmentSlot::OffHand, inventory, Area::Offhand, 0),
        ];
        self.send_packet(EntityEquipment {
            entity_id: network_id.0,
            entries,
        });
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_entity_position(
        &mut self,
        network_id: NetworkId,
        position: Position,
        prev_position: PreviousPosition,
        on_ground: OnGround,
        prev_on_ground: PreviousOnGround,
        dimension: &EntityDimension,
        world: EntityWorld,
        dimensions: &Dimensions,
        gamemode: Option<Gamemode>,
        previous_gamemode: Option<PreviousGamemode>,
    ) {
        let another_dimension =
            self.world != Some(world) || self.dimension.as_ref() != Some(dimension);

        if self.network_id == Some(network_id) {
            // This entity is the client. Only update
            // the position if it has changed from the client's
            // known position or dimension/world has changed.
            if another_dimension {
                self.move_to_dimension(
                    dimension.clone(),
                    dimensions,
                    gamemode.unwrap(),
                    previous_gamemode.unwrap(),
                    world,
                );
            } else if Some(position) != self.client_known_position {
                self.update_own_position(position);
            }
        } else if !another_dimension {
            let no_change_yaw = (position.yaw - prev_position.0.yaw).abs() < 0.001;
            let no_change_pitch = (position.pitch - prev_position.0.pitch).abs() < 0.001;

            // If the entity jumps or falls we should send a teleport packet instead to keep relative movement in sync.
            if on_ground != prev_on_ground.0 {
                self.send_packet(EntityTeleport {
                    entity_id: network_id.0,
                    x: position.x,
                    y: position.y,
                    z: position.z,
                    yaw: position.yaw,
                    pitch: position.pitch,
                    on_ground: *on_ground,
                });

                return;
            }

            if no_change_yaw && no_change_pitch {
                self.send_packet(EntityPosition {
                    entity_id: network_id.0,
                    delta_x: ((position.x * 32.0 - prev_position.0.x * 32.0) * 128.0) as i16,
                    delta_y: ((position.y * 32.0 - prev_position.0.y * 32.0) * 128.0) as i16,
                    delta_z: ((position.z * 32.0 - prev_position.0.z * 32.0) * 128.0) as i16,
                    on_ground: on_ground.0,
                });
            } else {
                self.send_packet(EntityPositionAndRotation {
                    entity_id: network_id.0,
                    delta_x: ((position.x * 32.0 - prev_position.0.x * 32.0) * 128.0) as i16,
                    delta_y: ((position.y * 32.0 - prev_position.0.y * 32.0) * 128.0) as i16,
                    delta_z: ((position.z * 32.0 - prev_position.0.z * 32.0) * 128.0) as i16,
                    yaw: position.yaw,
                    pitch: position.pitch,
                    on_ground: on_ground.0,
                });

                // Needed for head orientation
                self.send_packet(EntityHeadLook {
                    entity_id: network_id.0,
                    head_yaw: position.yaw,
                });
            }
        }
    }

    pub fn send_keepalive(&self) {
        log::trace!("Sending keepalive to {}", self.username);
        self.send_packet(KeepAlive { id: 0 });
    }

    pub fn send_entity_animation(&self, network_id: NetworkId, animation: Animation) {
        if self.network_id == Some(network_id) {
            return;
        }
        self.send_packet(EntityAnimation {
            entity_id: network_id.0,
            animation,
        })
    }

    pub fn send_message(&self, message: ChatMessage) {
        let packet = chat_packet(message);
        self.send_packet(packet);
    }

    pub fn send_chat_message(&self, message: ChatMessage) {
        let packet = chat_packet(message);
        self.send_packet(packet);
    }

    /// Sends all the required packets to display the [`Title`]
    ///
    /// If both the `title` and the `sub_title` are set to `None`
    /// This will emit the [`Title::Hide`] packet.
    ///
    /// If the sum of `fade_in`, `stay` and `fade_out` is `0`
    /// This will emit the [`Title::Reset`] packet.
    pub fn send_title(&self, title: Title) {
        if title.title.is_none() && title.sub_title.is_none() {
            self.send_packet(ClearTitles { reset: false });
        } else if title.fade_in + title.stay + title.fade_out == 0 {
            self.send_packet(ClearTitles { reset: true });
        } else {
            if let Some(main_title) = title.title {
                self.send_packet(SetTitleText {
                    title_text: main_title.to_string(),
                });
            }

            if let Some(sub_title) = title.sub_title {
                self.send_packet(SetTitleSubtitle {
                    subtitle_text: sub_title.to_string(),
                })
            }

            self.send_packet(SetTitleTimes {
                fade_in: title.fade_in as i32,
                stay: title.stay as i32,
                fade_out: title.fade_out as i32,
            });
        }
    }

    /// Resets the title for the player, this removes
    /// the text from the screen.
    ///
    /// Not to be confused with [`Self::hide_title()`]
    pub fn reset_title(&self) {
        self.send_packet(ClearTitles { reset: true });
    }

    /// Hides the title for the player, this removes
    /// the text from the screen, but it will re-appear again
    /// if the set times packet is sent again.
    ///
    /// Not to be confused with [`Self::reset_title()`]
    pub fn hide_title(&self) {
        self.send_packet(ClearTitles { reset: false });
    }

    pub fn send_window_items(&self, window: &PlayerWindow) {
        log::trace!("Updating inventory for {}", self.username);
        let packet = WindowItems {
            window_id: 0,
            state_id: 0,
            items: window.inner().to_vec(),
            cursor_item: window.cursor_item().clone(),
        };
        self.send_packet(packet);
    }

    pub fn send_inventory_slot(&self, slot: i16, item: &InventorySlot) {
        log::trace!(
            "Setting inventory slot {} of {} to {:?}",
            slot,
            self.username,
            item
        );
        self.send_packet(SetSlot {
            window_id: 0,
            state_id: 0,
            slot,
            slot_data: item.clone(),
        });
    }

    pub fn send_particle(&self, particle: &Particle, long_distance: bool, position: &Position) {
        self.send_packet(packets::server::Particle {
            particle_kind: particle.kind,
            long_distance,
            x: position.x,
            y: position.y,
            z: position.z,
            offset_x: particle.offset_x,
            offset_y: particle.offset_y,
            offset_z: particle.offset_z,
            particle_data: 0.0,
            particle_count: particle.count,
        })
    }

    pub fn send_cursor_slot(&self, item: &InventorySlot) {
        log::trace!("Setting cursor slot of {} to {:?}", self.username, item);
        self.send_inventory_slot(-1, item);
    }

    pub fn send_player_model_flags(&self, network_id: NetworkId, model_flags: u8) {
        let mut entity_metadata = EntityMetadata::new();
        entity_metadata.set(17, model_flags);
        self.send_packet(SendEntityMetadata {
            entity_id: network_id.0,
            entries: entity_metadata,
        });
    }

    pub fn send_entity_metadata(&self, network_id: NetworkId, metadata: EntityMetadata) {
        if self.network_id == Some(network_id) {
            return;
        }
        self.send_packet(SendEntityMetadata {
            entity_id: network_id.0,
            entries: metadata,
        });
    }

    pub fn send_abilities(&self, abilities: &libcraft::anvil::player::PlayerAbilities) {
        let mut bitfield = 0;
        if abilities.invulnerable {
            bitfield |= 1 << 0;
        }
        if abilities.is_flying {
            bitfield |= 1 << 1;
        }
        if abilities.may_fly {
            bitfield |= 1 << 2;
        }
        if abilities.instabreak {
            bitfield |= 1 << 3;
        }
        self.send_packet(PlayerAbilities {
            flags: bitfield,
            flying_speed: abilities.fly_speed,
            fov_modifier: abilities.walk_speed,
        });
    }

    pub fn send_hotbar_slot(&self, slot: u8) {
        self.send_packet(HeldItemChange { slot });
    }

    pub fn change_gamemode(&self, gamemode: Gamemode) {
        self.send_packet(ChangeGameState {
            state_change: GameStateChange::ChangeGamemode { gamemode },
        })
    }

    fn register_entity(&mut self, network_id: NetworkId) {
        self.sent_entities.insert(network_id);
    }

    fn send_packet<P: Into<ServerPlayPacket>>(&self, packet: P) {
        let packet = packet.into();
        log::trace!("Sending packet {} to {}", type_name::<P>(), self.username);
        let _ = self.packets_to_send.try_send(packet);
    }

    pub fn disconnect(&mut self, reason: impl Into<Text>) {
        self.disconnected = true;
        self.send_packet(Disconnect {
            reason: reason.into().to_string(),
        });
    }

    pub fn dimension(&self) -> &Option<EntityDimension> {
        &self.dimension
    }

    pub fn world(&self) -> &Option<EntityWorld> {
        &self.world
    }
}

fn equipment_entry(
    slot: EquipmentSlot,
    inventory: &Inventory,
    area: Area,
    index: usize,
) -> EquipmentEntry {
    EquipmentEntry {
        slot,
        item: InventorySlot::from(
            inventory
                .item(area, index)
                .map(|item| item.clone().into_option())
                .flatten(),
        ),
    }
}

fn chat_packet(message: ChatMessage) -> packets::server::ChatMessage {
    packets::server::ChatMessage {
        message: message.text().to_string(),
        position: match message.kind() {
            ChatKind::PlayerChat => ChatPosition::Chat,
            ChatKind::System => ChatPosition::SystemMessage,
            ChatKind::AboveHotbar => ChatPosition::Hotbar,
        },
        sender: Uuid::default(),
    }
}
