use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Stray;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Stray).with(Stray)
}
