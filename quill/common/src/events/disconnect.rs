use serde::{Deserialize, Serialize};

use libcraft_text::Text;

/// Disconnects the player with a specified kick message
#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, derive_more::Deref, derive_more::DerefMut,
)]
pub struct DisconnectEvent(Text);

impl DisconnectEvent {
    pub fn new(message: impl Into<Text>) -> DisconnectEvent {
        DisconnectEvent(message.into())
    }
}
