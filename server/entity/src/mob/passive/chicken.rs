use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Chicken;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Chicken).with(Chicken)
}
