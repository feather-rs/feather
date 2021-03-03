use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InteractionType {
    Interact,
    Attack,
    InteractAt,
}
