use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct ZombieVillager;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::ZombieVillager).with(ZombieVillager)
}
