//! Quill, Feather's plugin API.

mod chunk_lock;
pub mod components;
/// Marker components for each specific entity.
pub mod entities;
pub mod events;
mod game;
mod plugin;

#[doc(inline)]
pub use vane::{Entities, Entity, EntityBuilder, EntityRef, Resources, SysResult};

pub use game::Game;
pub use plugin::{Plugin, Setup};

#[doc(inline)]
pub use chunk_lock::{ChunkHandle, ChunkLock};
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
