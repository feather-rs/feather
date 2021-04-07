use serde::{Deserialize, Serialize};

pub enum EntityHungerEvent {
    RestoreSaturation(u32),
    DepleteSaturation(u32),

    RestoreFood(u32),
    DepleteFood(u32),
}

pub mod entity_exhaustion_event_kind {
    use super::{Deserialize, Serialize};

    ///
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Attack;

    ///
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct BlockMined;

    ///
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Crouch;

    ///
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Damaged;

    ///
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct HungerEffect(u32);

    ///
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Jump;

    ///
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct JumpSprint;

    ///
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Regen;

    ///
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Sprint;

    ///
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Swim;
}
