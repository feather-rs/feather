use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Donkey;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Donkey).with(Donkey)
}
