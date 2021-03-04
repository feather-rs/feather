#![allow(warnings)]

use std::{cell::RefCell, rc::Rc};

use feather_common::Game;
use feather_ecs::{HasResources, SysResult};
use feather_plugin_host_macros::host_function;

use crate::{
    context::{PluginContext, PluginPtr, PluginPtrMut},
    PluginId, PluginManager,
};

#[host_function]
pub fn register_system(
    cx: &PluginContext,
    data_ptr: PluginPtrMut<u8>,
    name_ptr: PluginPtr<u8>,
    name_len: u32,
) -> anyhow::Result<()> {
    let name = cx.read_string(name_ptr, name_len)?;

    let game = cx.game_mut();
    game.system_executor
        .borrow_mut()
        .add_system_with_name(plugin_system(cx.plugin_id(), data_ptr), &name);

    Ok(())
}

fn plugin_system(id: PluginId, data_ptr: PluginPtrMut<u8>) -> impl FnMut(&mut Game) -> SysResult {
    move |game: &mut Game| {
        let plugin_manager = Rc::clone(&*game.resources.get::<Rc<RefCell<PluginManager>>>()?);
        let plugin_manager = plugin_manager.borrow();
        let plugin = plugin_manager.plugin(id);
        if let Some(plugin) = plugin {
            plugin.run_system(game, data_ptr)?;
        }

        Ok(())
    }
}
