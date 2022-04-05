use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Vex;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Vex).with(Vex)
}
