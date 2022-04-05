use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Cow;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Cow).with(Cow)
}
