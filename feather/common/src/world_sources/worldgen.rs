use std::sync::Arc;

use anyhow::bail;
use libcraft::{
    anvil::level::SuperflatGeneratorOptions, biome::BiomeList, dimension::DimensionInfo,
    WorldHeight,
};
use quill::{
    saveload::{
        worldgen::WorldGeneratorWorldSource, EmptyWorldSource, WorldSource, WorldSourceFactory,
    },
    Game,
};
use worldgen::SuperflatWorldGenerator;

#[derive(Debug, serde::Deserialize)]
struct Params {
    generator: String,
}

pub struct WorldgenWorldSourceFactory;

impl WorldSourceFactory for WorldgenWorldSourceFactory {
    fn create_world_source(
        &self,
        game: &dyn Game,
        params: &toml::Value,
        dimension_info: &DimensionInfo,
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

        Ok(Box::new(WorldGeneratorWorldSource::new(
            EmptyWorldSource::default(),
            generator,
            game.compute_pool(),
        )))
    }
}
