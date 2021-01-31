use std::{cell::Cell, io::Cursor, sync::Arc};

use base::{Chunk, ChunkPosition, Gamemode, Position};
use flume::{Receiver, Sender};
use parking_lot::RwLock;
use protocol::{
    packets::server::{JoinGame, PlayerPositionAndLook, PluginMessage, UnloadChunk},
    ClientPlayPacket, Nbt, ServerPlayPacket,
};
use vec_arena::Arena;

use crate::{initial_handler::NewPlayer, Options};

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

    teleport_id_counter: Cell<i32>,
}

impl Client {
    pub fn new(player: NewPlayer, options: Arc<Options>) -> Self {
        Self {
            packets_to_send: player.packets_to_send,
            received_packets: player.received_packets,
            options,
            teleport_id_counter: Cell::new(0),
        }
    }

    pub fn send_join_game(&self, entity_id: i32, gamemode: Gamemode) {
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
            entity_id,
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
        self.send_packet(PluginMessage {
            channel: channel.into(),
            data: data.into(),
        })
    }

    pub fn update_own_position(&self, new_position: Position) {
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

    pub fn send_chunk(&self, chunk: &Arc<RwLock<Chunk>>) {
        todo!();
    }

    pub fn unload_chunk(&self, pos: ChunkPosition) {
        self.send_packet(UnloadChunk {
            chunk_x: pos.x,
            chunk_z: pos.z,
        });
    }

    fn send_packet(&self, packet: impl Into<ServerPlayPacket>) {
        let _ = self.packets_to_send.try_send(packet.into());
    }
}
