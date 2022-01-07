use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PluginMessageReceiveEvent {
    pub channel: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PluginMessageSendEvent {
    pub channel: String,
    pub data: Vec<u8>,
}
