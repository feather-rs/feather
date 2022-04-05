use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct ZombieVillager;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::ZombieVillager).with(ZombieVillager)
}
