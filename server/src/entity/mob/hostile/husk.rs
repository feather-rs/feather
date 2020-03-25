use crate::entity::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Husk;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Husk).with(Husk)
}
