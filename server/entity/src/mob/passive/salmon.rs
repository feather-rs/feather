use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Salmon;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Salmon).with(Salmon)
}
