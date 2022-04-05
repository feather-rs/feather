use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct PolarBear;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::PolarBear).with(PolarBear)
}
