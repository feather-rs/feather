//! Data pack implementation for Feather.
//!
//! Data packs can register loot tables, recipes, advancements, functions,
//! etc. This implementation aims to be compatible with vanilla data packs.
//!
//! This crate also downloads vanilla JARs and assets
//! at startup; see `download_vanilla_assets`.

use std::path::Path;

use ahash::AHashMap;
use id::ParseError;
use serde::Deserialize;
use smartstring::{LazyCompact, SmartString};

mod id;
pub use id::NamespacedId;

mod serde_helpers;
pub(crate) use serde_helpers::*;

pub mod tag;
use tag::LoopError;
pub use tag::{TagRegistry, TagRegistryBuilder};

pub mod recipe;
pub use recipe::RecipeRegistry;

/// The default namespace for resource locations (NamespacedIds).
pub const DEFAULT_NAMESPACE: &str = "minecraft";

use thiserror::Error;
#[derive(Error, Debug)]
pub enum TagLoadError {
    #[error("invalid namespaced id: {0}")]
    Parse(#[from] ParseError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("io error: {0}")]
    WalkDir(#[from] walkdir::Error),
    #[error("loop detected when parsing tags: {0}")]
    FoundLoop(#[from] LoopError),
    #[error("invalid tag link: {0} references {1}")]
    InvalidLink(NamespacedId, NamespacedId),
    #[error("json parsing error: {0}")]
    Json(#[from] serde_json::Error),
}
#[derive(Error, Debug)]
pub enum RecipeLoadError {
    #[error("invalid namespaced id: {0}")]
    Parse(#[from] ParseError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("json parsing error: {0}")]
    Json(#[from] serde_json::Error),
}

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
#[derive(Default)]
pub struct Datapack {
    pub advancements: (),
    pub loot_tables: (),
    pub recipes: (),
    pub structures: (),
    pub tags: TagRegistry,
}

impl Datapack {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn from_folder(dir: &Path) -> Self {
        assert!(dir.is_dir(), "not a directory");
        todo!()
    }
}
