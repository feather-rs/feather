use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Mule;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Mule).with(Mule)
}
