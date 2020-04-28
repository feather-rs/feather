use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Skeleton;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Skeleton).with(Skeleton)
}
