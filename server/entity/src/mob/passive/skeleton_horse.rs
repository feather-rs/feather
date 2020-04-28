use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct SkeletonHorse;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Horse).with(SkeletonHorse)
}
