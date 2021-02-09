use serde::{Deserialize, Serialize};

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
}
