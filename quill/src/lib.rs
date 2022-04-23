//! Quill, Feather's plugin API.

pub mod chat;
mod chunk_lock;
pub mod components;
/// Marker components for each specific entity.
pub mod entities;
pub mod events;
pub mod game;
mod plugin;
pub mod saveload;
pub mod threadpool;
pub mod world;

#[doc(inline)]
pub use vane::{Component, Entities, Entity, EntityBuilder, EntityRef, Resources, SysResult};

#[doc(inline)]
pub use chat::ChatBox;
#[doc(inline)]
pub use chunk_lock::{ChunkHandle, ChunkLock};
#[doc(inline)]
pub use game::Game;
pub use plugin::{Plugin, PluginInfo, Setup};
#[doc(inline)]
pub use world::{World, WorldId};

#[doc(inline)]
pub use libcraft::{
    blocks::{block_data, BlockData, BlockKind, BlockState},
    chunk::{Chunk, ChunkSection},
    inventory::Inventory,
    items::{
        Enchantment, EnchantmentKind, InventorySlot, Item, ItemStack, ItemStackBuilder,
        ItemStackError, ItemStackMeta,
    },
    text::{Text, TextComponentBuilder},
    BlockPosition, ChunkPosition, Position, CHUNK_WIDTH,
};

pub extern crate libcraft;
