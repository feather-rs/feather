use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Wolf;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Wolf).with(Wolf)
}
