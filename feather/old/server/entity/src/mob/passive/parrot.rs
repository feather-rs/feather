use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Parrot;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Parrot).with(Parrot)
}
