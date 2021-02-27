use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Blaze;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Blaze).with(Blaze)
}
