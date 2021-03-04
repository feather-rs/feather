use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Villager;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Villager).with(Villager)
}
