use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Zombie;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Zombie).with(Zombie)
}
