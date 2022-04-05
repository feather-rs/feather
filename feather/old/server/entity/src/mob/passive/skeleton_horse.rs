use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct SkeletonHorse;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Horse).with(SkeletonHorse)
}
