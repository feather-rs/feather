use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Evoker;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::EvocationIllager).with(Evoker)
}
