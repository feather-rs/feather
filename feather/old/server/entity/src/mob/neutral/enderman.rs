use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Enderman;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Enderman).with(Enderman)
}
