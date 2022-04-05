use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct TropicalFish;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::TropicalFish).with(TropicalFish)
}
