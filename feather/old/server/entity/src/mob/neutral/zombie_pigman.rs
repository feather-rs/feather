use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct ZombiePigman;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::PigZombie).with(ZombiePigman)
}
