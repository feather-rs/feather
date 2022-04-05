use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Witch;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Witch).with(Witch)
}
