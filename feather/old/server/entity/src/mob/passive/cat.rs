use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Cat;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Ocelot).with(Cat)
}
