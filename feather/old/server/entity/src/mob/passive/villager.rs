use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Villager;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Villager).with(Villager)
}
