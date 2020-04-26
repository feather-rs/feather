use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Silverfish;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Silverfish).with(Silverfish)
}
