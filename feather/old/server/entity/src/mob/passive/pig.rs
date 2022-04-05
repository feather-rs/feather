use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Pig;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Pig).with(Pig)
}
