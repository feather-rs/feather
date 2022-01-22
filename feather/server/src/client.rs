use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    io::Cursor,
    sync::Arc,
};

use ahash::AHashSet;
use flume::{Receiver, Sender};
use slab::Slab;
use uuid::Uuid;

use base::{
    BlockId, ChunkHandle, ChunkPosition, EntityKind, EntityMetadata, Gamemode, Position,
    ProfileProperty, Text, ValidBlockPosition,
};
use common::{
    chat::{ChatKind, ChatMessage},
    Window,
};
use libcraft_items::InventorySlot;
use packets::server::{Particle, SetSlot, SpawnLivingEntity, UpdateLight, WindowConfirmation};
use protocol::packets::server::{
    ChangeGameState, EntityPosition, EntityPositionAndRotation, EntityTeleport, GameStateChange,
    HeldItemChange, PlayerAbilities,
};
use protocol::{
    packets::{
        self,
        server::{
            AddPlayer, Animation, BlockChange, ChatPosition, ChunkData, ChunkDataKind,
            DestroyEntities, Disconnect, EntityAnimation, EntityHeadLook, JoinGame, KeepAlive,
            PlayerInfo, PlayerPositionAndLook, PluginMessage, SendEntityMetadata, SpawnPlayer,
            Title, UnloadChunk, UpdateViewPosition, WindowItems,
        },
    },
    ClientPlayPacket, Nbt, ProtocolVersion, ServerPlayPacket, Writeable,
};
use quill_common::components::{OnGround, PreviousGamemode};

use crate::{
    entities::{PreviousOnGround, PreviousPosition},
    initial_handler::NewPlayer,
    network_id_registry::NetworkId,
    Options,
};

/// Max number of chunks to send to a client per tick.
const MAX_CHUNKS_PER_TICK: usize = 10;

/// ID of a client. Can be reused.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClientId(usize);

/// Stores all `Client`s.
#[derive(Default)]
pub struct Clients {
    slab: Slab<Client>,
}

impl Clients {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, client: Client) -> ClientId {
        ClientId(self.slab.insert(client))
    }

    pub fn remove(&mut self, id: ClientId) -> Option<Client> {
        self.slab.try_remove(id.0)
    }

    pub fn get(&self, id: ClientId) -> Option<&Client> {
        self.slab.get(id.0)
    }

    pub fn get_mut(&mut self, id: ClientId) -> Option<&mut Client> {
        self.slab.get_mut(id.0)
    }

    pub fn iter(&self) -> impl Iterator<Item = &'_ Client> + '_ {
        self.slab.iter().map(|(_i, client)| client)
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

    teleport_id_counter: Cell<i32>,

    network_id: Option<NetworkId>,
    sent_entities: RefCell<AHashSet<NetworkId>>,

    knows_position: Cell<bool>,
    known_chunks: RefCell<AHashSet<ChunkPosition>>,

    chunk_send_queue: RefCell<VecDeque<ChunkData>>,

    /// The previous own position sent by the client.
    /// Used to detect when we need to teleport the client.
    client_known_position: Cell<Option<Position>>,

    disconnected: Cell<bool>,
}

impl Client {
    pub fn new(player: NewPlayer, options: Arc<Options>) -> Self {
        Self {
            packets_to_send: player.packets_to_send,
            received_packets: player.received_packets,
            options,
            username: player.username,
            teleport_id_counter: Cell::new(0),
            network_id: None,
            profile: player.profile,
            uuid: player.uuid,
            sent_entities: RefCell::new(AHashSet::new()),
            knows_position: Cell::new(false),
            known_chunks: RefCell::new(AHashSet::new()),
            chunk_send_queue: RefCell::new(VecDeque::new()),
            client_known_position: Cell::new(None),
            disconnected: Cell::new(false),
        }
    }

    pub fn set_client_known_position(&self, pos: Position) {
        self.client_known_position.set(Some(pos));
    }

    pub fn client_known_position(&self) -> Option<Position> {
        self.client_known_position.get()
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
        self.received_packets.is_disconnected() || self.disconnected.get()
    }

    pub fn known_chunks(&self) -> usize {
        self.known_chunks.borrow().len()
    }

    pub fn knows_own_position(&self) -> bool {
        self.knows_position.get()
    }

    pub fn tick(&self) {
        let num_to_send = MAX_CHUNKS_PER_TICK.min(self.chunk_send_queue.borrow().len());
        for packet in self.chunk_send_queue.borrow_mut().drain(0..num_to_send) {
            log::trace!(
                "Sending chunk at {:?} to {}",
                packet.chunk.read().position(),
                self.username
            );
            let chunk = Arc::clone(&packet.chunk);
            self.send_packet(UpdateLight { chunk });
            self.send_packet(packet);
        }
    }

    /// Returns whether the entity with the given ID
    /// is currently loaded on the client.
    pub fn is_entity_loaded(&self, network_id: NetworkId) -> bool {
        self.sent_entities.borrow().contains(&network_id)
    }

    pub fn set_network_id(&mut self, network_id: NetworkId) {
        self.network_id = Some(network_id);
    }

    pub fn send_join_game(&self, gamemode: Gamemode, previous_gamemode: PreviousGamemode) {
        log::trace!("Sending Join Game to {}", self.username);
        // Use the dimension codec sent by the default vanilla server. (Data acquired via tools/proxy)
        let dimension_codec = nbt::Blob::from_reader(&mut Cursor::new(include_bytes!(
            "../../../assets/dimension_codec.nbt"
        )))
        .expect("dimension codec asset is malformed");
        let dimension = nbt::Blob::from_reader(&mut Cursor::new(include_bytes!(
            "../../../assets/dimension.nbt"
        )))
        .expect("dimension asset is malformed");

        self.send_packet(JoinGame {
            entity_id: self.network_id.expect("No network id! Use client.set_network_id(NetworkId) before calling this method.").0,
            is_hardcore: false,
            gamemode,
            previous_gamemode,
            world_names: vec!["world".to_owned()],
            dimension_codec: Nbt(dimension_codec),
            dimension: Nbt(dimension),
            world_name: "world".to_owned(),
            hashed_seed: 0,
            max_players: 0,
            view_distance: self.options.view_distance as i32,
            reduced_debug_info: false,
            enable_respawn_screen: true,
            is_debug: false,
            is_flat: false,
        });
    }

    pub fn send_brand(&self) {
        let mut data = Vec::new();
        "Feather"
            .to_owned()
            .write(&mut data, ProtocolVersion::V1_16_2)
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

    pub fn update_own_position(&self, new_position: Position) {
        log::trace!(
            "Updating position of {} to {:?}",
            self.username,
            new_position
        );
        self.send_packet(PlayerPositionAndLook {
            x: new_position.x,
            y: new_position.y,
            z: new_position.z,
            yaw: new_position.yaw,
            pitch: new_position.pitch,
            flags: 0,
            teleport_id: self.teleport_id_counter.get(),
        });
        self.teleport_id_counter
            .set(self.teleport_id_counter.get() + 1);
        self.knows_position.set(true);
        self.client_known_position.set(Some(new_position));
    }

    pub fn update_own_chunk(&self, pos: ChunkPosition) {
        log::trace!("Updating chunk position of {} to {:?}", self.username, pos);
        self.send_packet(UpdateViewPosition {
            chunk_x: pos.x,
            chunk_z: pos.z,
        });
    }

    pub fn send_chunk(&self, chunk: &ChunkHandle) {
        self.chunk_send_queue.borrow_mut().push_back(ChunkData {
            chunk: Arc::clone(chunk),
            kind: ChunkDataKind::LoadChunk,
        });
        self.known_chunks
            .borrow_mut()
            .insert(chunk.read().position());
    }

    pub fn overwrite_chunk_sections(&self, chunk: &ChunkHandle, sections: Vec<usize>) {
        self.send_packet(ChunkData {
            chunk: Arc::clone(chunk),
            kind: ChunkDataKind::OverwriteChunk { sections },
        });
    }

    pub fn send_block_change(&self, position: ValidBlockPosition, new_block: BlockId) {
        self.send_packet(BlockChange {
            position,
            block: new_block,
        });
    }

    pub fn unload_chunk(&self, pos: ChunkPosition) {
        log::trace!("Unloading chunk at {:?} on {}", pos, self.username);
        self.send_packet(UnloadChunk {
            chunk_x: pos.x,
            chunk_z: pos.z,
        });
        self.known_chunks.borrow_mut().remove(&pos);
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

    pub fn unload_entity(&self, id: NetworkId) {
        log::trace!("Unloading {:?} on {}", id, self.username);
        self.sent_entities.borrow_mut().remove(&id);
        self.send_packet(DestroyEntities {
            entity_ids: vec![id.0.into()],
        });
    }

    pub fn send_player(&self, network_id: NetworkId, uuid: Uuid, pos: Position) {
        log::trace!("Sending {:?} to {}", uuid, self.username);
        assert!(!self.sent_entities.borrow().contains(&network_id));
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

    pub fn update_entity_position(
        &self,
        network_id: NetworkId,
        position: Position,
        prev_position: PreviousPosition,
        on_ground: OnGround,
        prev_on_ground: PreviousOnGround,
    ) {
        if self.network_id == Some(network_id) {
            // This entity is the client. Only update
            // the position if it has changed from the client's
            // known position.
            if Some(position) != self.client_known_position.get() {
                self.update_own_position(position);
            }
            return;
        }

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
    pub fn send_title(&self, title: base::Title) {
        if title.title.is_none() && title.sub_title.is_none() {
            self.send_packet(Title::Hide);
        } else if title.fade_in + title.stay + title.fade_out == 0 {
            self.send_packet(Title::Reset);
        } else {
            if let Some(main_title) = title.title {
                self.send_packet(Title::SetTitle {
                    text: main_title.to_string(),
                });
            }

            if let Some(sub_title) = title.sub_title {
                self.send_packet(Title::SetSubtitle {
                    text: sub_title.to_string(),
                })
            }

            self.send_packet(Title::SetTimesAndDisplay {
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
        self.send_packet(Title::Reset);
    }

    /// Hides the title for the player, this removes
    /// the text from the screen, but it will re-appear again
    /// if the set times packet is sent again.
    ///
    /// Not to be confused with [`Self::reset_title()`]
    pub fn hide_title(&self) {
        self.send_packet(Title::Hide);
    }

    pub fn confirm_window_action(&self, window_id: u8, action_number: i16, is_accepted: bool) {
        self.send_packet(WindowConfirmation {
            window_id,
            action_number,
            is_accepted,
        });
    }

    pub fn send_window_items(&self, window: &Window) {
        log::trace!("Updating window for {}", self.username);
        let packet = WindowItems {
            window_id: 0,
            items: window.inner().to_vec(),
        };
        self.send_packet(packet);
    }

    pub fn set_slot(&self, slot: i16, item: &InventorySlot) {
        log::trace!("Setting slot {} of {} to {:?}", slot, self.username, item);
        self.send_packet(SetSlot {
            window_id: 0,
            slot,
            slot_data: item.clone(),
        });
    }

    pub fn send_particle(&self, particle: &base::Particle, position: &Position) {
        self.send_packet(Particle {
            particle_kind: particle.kind,
            long_distance: true,
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

    pub fn set_cursor_slot(&self, item: &InventorySlot) {
        log::trace!("Setting cursor slot of {} to {:?}", self.username, item);
        self.set_slot(-1, item);
    }

    pub fn send_player_model_flags(&self, netowrk_id: NetworkId, model_flags: u8) {
        let mut entity_metadata = EntityMetadata::new();
        entity_metadata.set(16, model_flags);
        self.send_packet(SendEntityMetadata {
            entity_id: netowrk_id.0,
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

    pub fn send_abilities(&self, abilities: &base::anvil::player::PlayerAbilities) {
        let mut bitfield = 0;
        if *abilities.invulnerable {
            bitfield |= 1 << 0;
        }
        if *abilities.is_flying {
            bitfield |= 1 << 1;
        }
        if *abilities.may_fly {
            bitfield |= 1 << 2;
        }
        if *abilities.instabreak {
            bitfield |= 1 << 3;
        }
        self.send_packet(PlayerAbilities {
            flags: bitfield,
            flying_speed: *abilities.fly_speed,
            fov_modifier: *abilities.walk_speed,
        });
    }

    pub fn set_hotbar_slot(&self, slot: u8) {
        self.send_packet(HeldItemChange { slot });
    }

    pub fn change_gamemode(&self, gamemode: Gamemode) {
        self.send_packet(ChangeGameState {
            state_change: GameStateChange::ChangeGamemode { gamemode },
        })
    }

    fn register_entity(&self, network_id: NetworkId) {
        self.sent_entities.borrow_mut().insert(network_id);
    }

    fn send_packet(&self, packet: impl Into<ServerPlayPacket>) {
        let _ = self.packets_to_send.try_send(packet.into());
    }

    pub fn disconnect(&self, reason: &str) {
        self.disconnected.set(true);
        self.send_packet(Disconnect {
            reason: Text::from(reason.to_owned()).to_string(),
        });
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
