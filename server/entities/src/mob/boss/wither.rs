use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Wither;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Wither).with(Wither)
}
