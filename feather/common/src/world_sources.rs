//! Built-in world sources.

use quill::saveload::{EmptyWorldSource, WorldSourceFactory};

pub mod worldgen;

pub struct EmptyWorldSourceFactory;

impl WorldSourceFactory for EmptyWorldSourceFactory {
    fn create_world_source(
        &self,
        _game: &dyn quill::Game,
        _params: &toml::Value,
        _dimension_info: &libcraft::dimension::DimensionInfo,
        _world_id: quill::WorldId,
    ) -> anyhow::Result<Box<dyn quill::saveload::WorldSource>> {
        Ok(Box::new(EmptyWorldSource::default()))
    }
}
