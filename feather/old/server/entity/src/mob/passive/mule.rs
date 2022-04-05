use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Mule;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Mule).with(Mule)
}
