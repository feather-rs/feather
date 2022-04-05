use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct ElderGuardian;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::ElderGuardian).with(ElderGuardian)
}
