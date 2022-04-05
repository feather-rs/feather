use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Mooshroom;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::MushroomCow).with(Mooshroom)
}
