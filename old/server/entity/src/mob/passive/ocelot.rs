use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Ocelot;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Ocelot).with(Ocelot)
}
