use std::convert::TryInto;

use feather_base::{BlockId, BlockPosition, ChunkPosition};
use feather_plugin_host_macros::host_function;
use quill_common::block::BlockGetResult;

use crate::context::PluginContext;

/// NB: `u32` has the same layout as `BlockGetResult`.
#[host_function]
pub fn block_get(cx: &PluginContext, x: i32, y: i32, z: i32) -> anyhow::Result<u32> {
    let pos = BlockPosition::new(x, y, z).try_into()?;

    let block = cx.game_mut().block(pos);
    let result = BlockGetResult::new(block.map(BlockId::vanilla_id));
    Ok(result.to_u32())
}

#[host_function]
pub fn block_set(cx: &PluginContext, x: i32, y: i32, z: i32, block_id: u16) -> anyhow::Result<u32> {
    let pos = BlockPosition::new(x, y, z).try_into()?;
    let block = BlockId::from_vanilla_id(block_id);

    let was_successful = cx.game_mut().set_block(pos, block);
    Ok(was_successful as u32)
}

#[host_function]
pub fn block_fill_chunk_section(
    cx: &PluginContext,
    chunk_x: i32,
    section_y: u32,
    chunk_z: i32,
    block_id: u16,
) -> anyhow::Result<u32> {
    let chunk_pos = ChunkPosition::new(chunk_x, chunk_z);
    let block = BlockId::from_vanilla_id(block_id);
    let was_successful = cx
        .game_mut()
        .fill_chunk_section(chunk_pos, section_y as usize, block);
    Ok(was_successful as u32)
}
