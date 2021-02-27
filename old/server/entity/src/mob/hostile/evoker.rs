use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Evoker;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::EvocationIllager).with(Evoker)
}
