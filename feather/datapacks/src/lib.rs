//! Data pack implementation for Feather.
//!
//! Data packs can register loot tables, recipes, advancements, functions,
//! etc. This implementation aims to be compatible with vanilla data packs.
//!
//! This crate also downloads vanilla JARs and assets
//! at startup; see `download_vanilla_assets`.

use ahash::AHashMap;
use serde::Deserialize;
use smartstring::{LazyCompact, SmartString};

mod vanilla;
pub use vanilla::download_vanilla_assets;

mod id;
pub use id::NamespacedId;

/// The default namespace for resource locations (NamespacedIds).
pub const DEFAULT_NAMESPACE: &str = "minecraft";

/// The pack.mcmeta file at the root of a datapack.
///
/// Formatted with JSON.
#[derive(Debug, Deserialize)]
pub struct PackMeta {
    pub pack_format: i32,
    pub description: String,
}

/// Stores all loaded data packs and their assets.
pub struct Datapacks {
    /// The metadata of loaded packs. Keyed by the datapack name.
    _meta: AHashMap<SmartString<LazyCompact>, PackMeta>,
}
