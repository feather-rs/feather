use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Creeper;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Creeper).with(Creeper)
}
