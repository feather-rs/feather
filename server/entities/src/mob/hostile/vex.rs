use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Vex;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Vex).with(Vex)
}
