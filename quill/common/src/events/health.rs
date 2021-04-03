use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityDamageEvent {
    pub amount: u32,
    pub final_amount: u32,
    pub is_cancelled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityRegenEvent {
    pub amount: u32,
    pub is_cancelled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EntityHealthEvent {
    Damage(EntityDamageEvent),
    Regen(EntityRegenEvent),
}
