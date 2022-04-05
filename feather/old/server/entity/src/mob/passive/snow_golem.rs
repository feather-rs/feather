use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct SnowGolem;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::SnowGolem).with(SnowGolem)
}
