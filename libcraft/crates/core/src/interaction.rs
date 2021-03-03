use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum InteractionType {
    Interact,
    Attack,
    InteractAt,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InteractHand {
    Main,
    Offhand,
}
