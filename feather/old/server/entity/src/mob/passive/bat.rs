use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Bat;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Bat).with(Bat)
}
