use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Shulker;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Shulker).with(Shulker)
}
