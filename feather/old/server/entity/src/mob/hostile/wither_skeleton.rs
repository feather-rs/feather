use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct WitherSkeleton;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::WitherSkeleton).with(WitherSkeleton)
}
