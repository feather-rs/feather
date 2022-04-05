use crate::{mob, MobKind};
use fvane::EntityBuilder;

pub struct Endermite;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Endermite).with(Endermite)
}
