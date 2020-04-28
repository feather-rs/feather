use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Cow;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Cow).with(Cow)
}
