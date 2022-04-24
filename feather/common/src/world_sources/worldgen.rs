use std::sync::Arc;

use anyhow::{bail, Context};
use libcraft::{
    anvil::level::SuperflatGeneratorOptions, biome::BiomeList, dimension::DimensionInfo,
    WorldHeight,
};
use quill::{
    saveload::{worldgen::WorldGeneratorWorldSource, WorldSource, WorldSourceFactory},
    Game, WorldId,
};
use worldgen::SuperflatWorldGenerator;

#[derive(Debug, serde::Deserialize)]
struct Params {
    generator: String,
    inner: Inner,
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

        let generator = match params.generator.as_str() {
            "flat" => Arc::new(SuperflatWorldGenerator::new(
                SuperflatGeneratorOptions::default(),
                game.resources().get::<Arc<BiomeList>>()?.clone(),
                WorldHeight(dimension_info.info.height as usize).into(),
                dimension_info.info.min_y,
            )),
            gen => bail!("unknown world generator '{}'", gen),
        };

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
