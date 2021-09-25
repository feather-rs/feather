use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Donkey;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Donkey).with(Donkey)
}
