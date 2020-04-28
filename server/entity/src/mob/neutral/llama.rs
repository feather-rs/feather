use crate::{mob, MobKind};
use fecs::EntityBuilder;

pub struct Llama;

pub fn create() -> EntityBuilder {
    mob::base(MobKind::Llama).with(Llama)
}
