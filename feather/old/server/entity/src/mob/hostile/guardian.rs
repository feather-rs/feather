use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Guardian;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Guardian).with(Guardian)
}
