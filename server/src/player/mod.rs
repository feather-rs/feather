//! Systems and components specific to player entities.

use crate::broadcasters::LastKnownPositions;
use crate::chunk_logic::ChunkHolder;
use crate::entity;
use crate::entity::{CreationPacketCreator, EntityId, Name, PreviousPosition, SpawnPacketCreator};
use crate::game::Game;
use crate::io::NewClientInfo;
use crate::network::Network;
use crate::p_inventory::{EntityInventory, InventoryUpdateEvent};
use crate::util::degrees_to_stops;
use feather_core::network::packets::{PlayerInfo, PlayerInfoAction, SpawnPlayer};
use feather_core::{Gamemode, ItemStack, Packet, Position};
use feather_items::Item;
use fecs::{Entity, EntityRef, World};
use mojang_api::ProfileProperty;
use uuid::Uuid;

pub const PLAYER_EYE_HEIGHT: f64 = 1.62;
