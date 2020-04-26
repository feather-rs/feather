use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Endermite;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Endermite).with(Endermite)
}
