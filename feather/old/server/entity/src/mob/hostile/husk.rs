use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Husk;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Husk).with(Husk)
}
