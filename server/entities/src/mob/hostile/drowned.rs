use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Drowned;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Drowned).with(Drowned)
}
