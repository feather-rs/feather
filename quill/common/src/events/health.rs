use serde::{Deserialize, Serialize};

/// Triggered by systems that need to mutate the health of an entity.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UpdateHealthEvent {
    Heal(u32),
    Harm(u32),
}
