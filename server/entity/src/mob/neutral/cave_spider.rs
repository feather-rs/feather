use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct CaveSpider;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::CaveSpider).with(CaveSpider)
}
