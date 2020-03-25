use crate::entity::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Turtle;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Turtle).with(Turtle)
}
