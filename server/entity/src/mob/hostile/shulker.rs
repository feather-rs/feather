use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Shulker;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Shulker).with(Shulker)
}
