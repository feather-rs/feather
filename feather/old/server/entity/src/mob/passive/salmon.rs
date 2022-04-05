use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Salmon;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Salmon).with(Salmon)
}
