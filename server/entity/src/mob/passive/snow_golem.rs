use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct SnowGolem;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::SnowGolem).with(SnowGolem)
}
