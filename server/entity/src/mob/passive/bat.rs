use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Bat;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Bat).with(Bat)
}
