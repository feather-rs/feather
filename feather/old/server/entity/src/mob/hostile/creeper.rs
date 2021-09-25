use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Creeper;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Creeper).with(Creeper)
}
