use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Sheep;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Sheep).with(Sheep)
}
