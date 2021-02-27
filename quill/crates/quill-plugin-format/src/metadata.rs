use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use target_lexicon::Triple;

/// A plugin's metadata, stored alongside its WASM module.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// Plugin name, no spaces
    pub name: String,
    /// Plugin identifier (crate name), snake_case or kebab-case
    pub identifier: String,
    /// Plugin version
    pub version: String,
    /// `quill` version used to compile the plugin
    pub api_version: String,

    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,

    pub target: PluginTarget,
}

/// Type of a plugin
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginTarget {
    Wasm,
    Native {
        /// The target the plugin has been compiled to.
        #[serde_as(as = "DisplayFromStr")]
        target_triple: Triple,
    },
}
