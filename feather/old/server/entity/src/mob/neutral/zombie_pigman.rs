use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct ZombiePigman;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::PigZombie).with(ZombiePigman)
}
