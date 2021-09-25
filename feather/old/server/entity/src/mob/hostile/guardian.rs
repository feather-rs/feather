use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Guardian;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Guardian).with(Guardian)
}
