use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Rabbit;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Rabbit).with(Rabbit)
}
