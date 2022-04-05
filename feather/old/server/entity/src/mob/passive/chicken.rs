use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Chicken;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Chicken).with(Chicken)
}
