use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Wolf;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Wolf).with(Wolf)
}
