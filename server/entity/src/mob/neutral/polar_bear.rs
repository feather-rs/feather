use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct PolarBear;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::PolarBear).with(PolarBear)
}
