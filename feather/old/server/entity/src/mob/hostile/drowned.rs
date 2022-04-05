use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Drowned;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Drowned).with(Drowned)
}
