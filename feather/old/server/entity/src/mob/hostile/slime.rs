use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Slime;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Slime).with(Slime)
}
