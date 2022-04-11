#[doc(inline)]
pub use libcraft_anvil as anvil;
#[doc(inline)]
pub use libcraft_blocks as blocks;
#[doc(inline)]
pub use libcraft_chunk as chunk;
#[doc(inline)]
pub use libcraft_core::*;
#[doc(inline)]
pub use libcraft_inventory as inventory;
#[doc(inline)]
pub use libcraft_items as items;
#[doc(inline)]
pub use libcraft_particles as particles;
#[doc(inline)]
pub use libcraft_text as text;

pub mod dimension;
pub mod entity_metadata;

#[doc(inline)]
pub use entity_metadata::{EntityMetadata, MetaEntry};

#[doc(inline)]
pub use libcraft_blocks::{BlockKind, BlockState};
#[doc(inline)]
pub use libcraft_chunk::{
    biome::{self, BiomeId},
    Chunk, ChunkSection,
};
#[doc(inline)]
pub use libcraft_inventory::{Area, Inventory, Window};
#[doc(inline)]
pub use libcraft_items::{Item, ItemStack, ItemStackBuilder, ItemStackMeta};
#[doc(inline)]
pub use libcraft_particles::{particle, Particle, ParticleKind};
#[doc(inline)]
pub use libcraft_text::{Text, TextComponentBuilder, Title};
