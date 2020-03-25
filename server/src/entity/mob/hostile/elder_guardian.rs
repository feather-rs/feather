use crate::entity::{mob, MobKind};
use fecs::EntityBuilder;

pub struct ElderGuardian;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::ElderGuardian).with(ElderGuardian)
}
