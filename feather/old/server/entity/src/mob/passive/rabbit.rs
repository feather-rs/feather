use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Rabbit;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Rabbit).with(Rabbit)
}
