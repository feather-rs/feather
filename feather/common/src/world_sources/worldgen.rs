use std::sync::Arc;

use anyhow::Context;
use libcraft::{
    anvil::level::SuperflatGeneratorOptions, biome::BiomeList, dimension::DimensionInfo, Sections,
    WorldHeight,
};
use quill::{
    saveload::{
        worldgen::{WorldGenerator, WorldGeneratorFactory, WorldGeneratorWorldSource},
        WorldSource, WorldSourceFactory,
    },
    Game, WorldId,
};
use worldgen::SuperflatWorldGenerator;

#[derive(Debug, serde::Deserialize)]
struct Params {
    generator: String,
    inner: Inner,
    #[serde(flatten)]
    params: toml::Value,
}

#[derive(Debug, serde::Deserialize)]
struct Inner {
    #[serde(rename = "type")]
    typ: String,
    #[serde(flatten)]
    params: toml::Value,
}

pub struct WorldgenWorldSourceFactory;

impl WorldSourceFactory for WorldgenWorldSourceFactory {
    fn create_world_source(
        &self,
        game: &dyn Game,
        params: &toml::Value,
        dimension_info: &DimensionInfo,
        world_id: WorldId,
    ) -> anyhow::Result<Box<dyn WorldSource>> {
        let params: Params = params.clone().try_into()?;

        let generator_factory = game.world_generator_factory(&params.generator)?;
        let generator = generator_factory
            .create_world_generator(
                game,
                &params.params,
                world_id,
                WorldHeight(dimension_info.info.height as usize).into(),
                dimension_info.info.min_y,
            )
            .with_context(|| {
                format!(
                    "failed to initialize world generator '{}'",
                    params.generator
                )
            })?;

        let inner = game
            .world_source_factory(&params.inner.typ)?
            .create_world_source(game, &params.inner.params, dimension_info, world_id)
            .with_context(|| {
                format!(
                    "failed to initialize inner '{}' world source",
                    params.inner.typ
                )
            })?;

        Ok(Box::new(WorldGeneratorWorldSource::new(
            inner,
            generator,
            game.compute_pool(),
        )))
    }
}

pub struct SuperflatWorldGeneratorFactory;

impl WorldGeneratorFactory for SuperflatWorldGeneratorFactory {
    fn create_world_generator(
        &self,
        game: &dyn Game,
        _params: &toml::Value,
        _world_id: WorldId,
        sections: Sections,
        min_y: i32,
    ) -> anyhow::Result<Arc<dyn WorldGenerator>> {
        Ok(Arc::new(SuperflatWorldGenerator::new(
            SuperflatGeneratorOptions::default(),
            game.resources().get::<Arc<BiomeList>>()?.clone(),
            sections,
            min_y,
        )))
    }
}
