use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Pufferfish;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Pufferfish).with(Pufferfish)
}
