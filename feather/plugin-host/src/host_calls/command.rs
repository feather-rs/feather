use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use feather_base::{BlockId, BlockPosition, ChunkPosition};
use feather_plugin_host_macros::host_function;
use quill_common::block::BlockGetResult;
use wasmer_wasi::types::print_right_set;

use crate::command_dispatcher::{CommandDescription, CommandDispatcher};
use crate::context::{PluginContext, PluginPtr, PluginPtrMut};

#[host_function]
pub fn register_command(
    cx: &PluginContext,
    regex: PluginPtr<u8>,
    regex_len: u32,
    command: PluginPtrMut<u8>,
) -> anyhow::Result<u32> {
    let dispatcher = Rc::clone(
        &*cx.game_mut()
            .resources
            .get::<Rc<RefCell<CommandDispatcher>>>()?,
    );
    let mut dispatcher = (*dispatcher).borrow_mut();

    let description = CommandDescription {
        plugin_id: cx.plugin_id(),
        regex: cx.read_string(regex, regex_len)?,
        callback: command,
    };
    dispatcher.add_command(description);
    Ok(true as u32)
}
