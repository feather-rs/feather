use std::convert::TryInto;

use crate::{entity::NetworkId, network::WorkerHandle};
use base::{anvil::level::LevelData, anvil::level::LevelGeneratorType, Gamemode};
use protocol::{packets::server::DimensionCodec, packets::server::JoinGame, Nbt, ServerPlayPacket};
use sha2::{Digest, Sha256};

pub mod messages {
    //! Re-export packet structs from feather-protocol.
    //! In the future, we might add our own message structs
    //! which wrap the raw packets.
}

/// Message received from a client.
///
/// A message roughly corresponds to a packet in the protocol,
/// but in some cases it may represent a higher layer of abstraction.
#[derive(Debug, Clone)]
pub enum Message {}

/// An abstraction layer over the Minecraft protocol
/// which bridges raw packets to functions and structs.
///
/// This layer exists mostly in case we add support for new
/// protocols in the future. For example, this abstraction
/// could allow for supporting multiple protocol versions in the future.
pub struct Session {
    inner: Box<dyn SessionImpl>,
    worker: WorkerHandle,
}

trait SessionImpl {
    /// Returns the packet used to join the client.
    fn join(
        &self,
        network_id: &NetworkId,
        gamemode: Gamemode,
        seed: u64,
        max_players: u32,
        view_distance: u8,
        level_type: LevelGeneratorType,
    ) -> ServerPlayPacket;
}

/// SessionImpl for vanilla 1.16.3.
struct VanillaSession;

impl SessionImpl for VanillaSession {
    fn join(
        &self,
        network_id: &NetworkId,
        gamemode: Gamemode,
        seed: u64,
        max_players: u32,
        view_distance: u8,
        level_type: LevelGeneratorType,
    ) -> ServerPlayPacket {
        JoinGame {
            entity_id: network_id.0,
            gamemode,
            previous_gamemode: gamemode, // TODO: what should this be?
            world_names: vec![String::from("world")], // no multiworld support yet
            dimension_codec: Nbt(DimensionCodec::overworld()),
            dimension: Nbt(nbt::Blob::new()), // TODO: what should this be?
            world_name: String::from("world"),
            hashed_seed: hash_seed(seed),
            max_players: max_players as i32,
            view_distance: view_distance as i32,
            reduced_debug_info: false,
            enable_respawn_screen: true,
            is_debug: false,
            is_flat: level_type == LevelGeneratorType::Flat,
        }
        .into()
    }
}

fn hash_seed(seed: u64) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(&seed.to_be_bytes()[..]);
    u64::from_be_bytes(
        hasher.finalize().as_slice()[..8]
            .try_into()
            .expect("slice => array"),
    )
}

#[cfg(test)]
mod tests {
    use protocol::VariantOf;

    use super::*;

    #[test]
    fn hash_some_seeds() {
        for seed in 0u64..10 {
            hash_seed(seed);
        }
    }

    #[test]
    fn create_join_game() {
        let packet = VanillaSession.join(
            &NetworkId(10),
            Gamemode::Survival,
            66,
            16,
            10,
            LevelGeneratorType::Amplified,
        );
        let packet = JoinGame::destructure(packet).unwrap();
        assert_eq!(packet.entity_id, 10);
        assert_eq!(packet.gamemode, Gamemode::Survival);
        assert_eq!(packet.previous_gamemode, Gamemode::Survival);
        assert_eq!(packet.view_distance, 10);
        assert_ne!(packet.hashed_seed, 66); // make sure hashed seed doesn't match seed
        assert_eq!(packet.max_players, 16);
        assert!(!packet.is_flat);
        assert!(!packet.is_debug);
        assert!(packet.enable_respawn_screen);
        assert!(!packet.reduced_debug_info);
    }

    #[test]
    fn create_join_game_for_flat_world() {
        let packet = VanillaSession.join(
            &NetworkId(10),
            Gamemode::Survival,
            66,
            16,
            10,
            LevelGeneratorType::Flat,
        );
        let packet = JoinGame::destructure(packet).unwrap();
        assert!(packet.is_flat);
    }
}
