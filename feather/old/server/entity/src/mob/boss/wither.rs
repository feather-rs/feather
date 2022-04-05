use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Wither;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Wither).with(Wither)
}
