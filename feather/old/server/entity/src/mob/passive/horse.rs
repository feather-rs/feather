use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Horse;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Horse).with(Horse)
}
