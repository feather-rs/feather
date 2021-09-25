use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Sheep;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Sheep).with(Sheep)
}
