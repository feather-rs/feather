use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Pufferfish;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Pufferfish).with(Pufferfish)
}
