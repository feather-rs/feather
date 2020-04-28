use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Mooshroom;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::MushroomCow).with(Mooshroom)
}
