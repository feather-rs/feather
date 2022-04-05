use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct WitherSkeleton;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::WitherSkeleton).with(WitherSkeleton)
}
