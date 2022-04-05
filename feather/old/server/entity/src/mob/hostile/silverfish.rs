use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Silverfish;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Silverfish).with(Silverfish)
}
