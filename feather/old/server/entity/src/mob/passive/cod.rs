use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Cod;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Cod).with(Cod)
}
