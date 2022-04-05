use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct CaveSpider;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::CaveSpider).with(CaveSpider)
}
