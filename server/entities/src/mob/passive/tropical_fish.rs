use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct TropicalFish;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::TropicalFish).with(TropicalFish)
}
