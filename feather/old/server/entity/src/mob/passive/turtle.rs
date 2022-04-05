use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Turtle;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Turtle).with(Turtle)
}
