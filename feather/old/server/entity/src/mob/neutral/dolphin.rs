use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Dolphin;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Dolphin).with(Dolphin)
}
