use feather_base::{BlockId, BlockPosition};
use feather_plugin_host_macros::host_function;
use quill_common::block::BlockGetResult;

use crate::context::PluginContext;

/// NB: `u32` has the same layout as `BlockGetResult`.
#[host_function]
pub fn block_get(cx: &PluginContext, x: i32, y: i32, z: i32) -> anyhow::Result<u32> {
    let pos = BlockPosition::new(x, y, z);

    let block = cx.game_mut().world.block_at(pos);
    let result = BlockGetResult::new(block.map(BlockId::vanilla_id));
    Ok(result.to_u32())
}

#[host_function]
pub fn block_set(cx: &PluginContext, x: i32, y: i32, z: i32, block_id: u16) -> anyhow::Result<u32> {
    let pos = BlockPosition::new(x, y, z);
    let block = BlockId::from_vanilla_id(block_id);

    let was_successful = cx.game_mut().world.set_block_at(pos, block);
    Ok(was_successful as u32)
}
