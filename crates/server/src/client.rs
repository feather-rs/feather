use std::{cell::Cell, io::Cursor, sync::Arc};

use base::{Chunk, ChunkPosition, Gamemode, Position};
use flume::{Receiver, Sender};
use parking_lot::RwLock;
use protocol::{
    packets::server::{
        ChunkData, JoinGame, PlayerPositionAndLook, PluginMessage, UnloadChunk, UpdateViewPosition,
    },
    ClientPlayPacket, Nbt, ServerPlayPacket,
};
use vec_arena::Arena;

use crate::{initial_handler::NewPlayer, network_id_registry::NetworkId, Options};

/// ID of a client. Can be reused.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClientId(usize);

/// Stores all `Client`s.
#[derive(Default)]
pub struct Clients {
    arena: Arena<Client>,
}

impl Clients {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, client: Client) -> ClientId {
        ClientId(self.arena.insert(client))
    }

    pub fn remove(&mut self, id: ClientId) -> Option<Client> {
        self.arena.remove(id.0)
    }

    pub fn get(&self, id: ClientId) -> Option<&Client> {
        self.arena.get(id.0)
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

    teleport_id_counter: Cell<i32>,

    network_id: NetworkId,
}

impl Client {
    pub fn new(player: NewPlayer, options: Arc<Options>, network_id: NetworkId) -> Self {
        Self {
            packets_to_send: player.packets_to_send,
            received_packets: player.received_packets,
            options,
            username: player.username,
            teleport_id_counter: Cell::new(0),
            network_id,
        }
    }

    pub fn network_id(&self) -> NetworkId {
        self.network_id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn received_packets<'a>(&'a self) -> impl Iterator<Item = ClientPlayPacket> + 'a {
        self.received_packets.try_iter()
    }

    pub fn send_join_game(&self, gamemode: Gamemode) {
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
            entity_id: self.network_id.0,
            is_hardcore: false,
            gamemode,
            previous_gamemode: 0,
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
        self.send_plugin_message("minecraft:brand", "Feather")
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
    }

    pub fn update_own_chunk(&self, pos: ChunkPosition) {
        log::trace!("Updating chunk position of {} to {:?}", self.username, pos);
        self.send_packet(UpdateViewPosition {
            chunk_x: pos.x,
            chunk_z: pos.z,
        });
    }

    pub fn send_chunk(&self, chunk: &Arc<RwLock<Chunk>>) {
        log::trace!(
            "Sending chunk at {:?} to {}",
            chunk.read().position(),
            self.username
        );
        /*self.send_packet(UpdateLight {
            chunk: Arc::clone(chunk),
        });*/
        self.send_packet(ChunkData {
            chunk: Arc::clone(chunk),
        });
    }

    pub fn unload_chunk(&self, pos: ChunkPosition) {
        log::trace!("Unloading chunk at {:?} on {}", pos, self.username);
        self.send_packet(UnloadChunk {
            chunk_x: pos.x,
            chunk_z: pos.z,
        });
    }

    fn send_packet(&self, packet: impl Into<ServerPlayPacket>) {
        let _ = self.packets_to_send.try_send(packet.into());
    }
}
