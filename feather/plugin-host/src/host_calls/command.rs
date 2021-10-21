use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::ManuallyDrop;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use commands::args::Func;
use commands::dispatcher::{Args, CommandDispatcher, CommandOutput, Completer, Fork};
use commands::node::CommandNode;
use commands::parser::ArgumentParser;
use wasmer::FromToNativeWasmType;

use feather_commands::CommandCtx;
use feather_plugin_host_macros::host_function;
use quill::command::{Caller, CommandContext};
use quill::Game;
use quill_common::EntityId;

use crate::context::{PluginContext, PluginPtr, PluginPtrMut};
use crate::PluginManager;

#[host_function]
pub fn modify_command_executor(
    cx: &PluginContext,
    nodes: PluginPtrMut<u8>,
    nodes_len: u32,
    nodes_cap: u32,
    executors: PluginPtrMut<u8>,
    executors_len: u32,
    executors_cap: u32,
    tab_completers: PluginPtrMut<u8>,
    tab_completers_len: u32,
    tab_completers_cap: u32,
    forks: PluginPtrMut<u8>,
    forks_len: u32,
    forks_cap: u32,
) -> anyhow::Result<()> {
    // SAFETY: Plugins should pass valid raw vec data.
    let (nodes, executors, tab_completers, forks) = unsafe {
        let nodes = Vec::from_raw_parts(
            nodes.as_native() as *mut CommandNode,
            nodes_len as usize,
            nodes_cap as usize,
        );
        let executors = Vec::from_raw_parts(
            executors.as_native() as *mut Box<dyn Func<CommandContext<()>, ()>>,
            executors_len as usize,
            executors_cap as usize,
        );
        let tab_completers = Vec::from_raw_parts(
            tab_completers.as_native() as *mut (String, Completer<CommandContext<()>>),
            tab_completers_len as usize,
            tab_completers_cap as usize,
        );
        let forks = Vec::from_raw_parts(
            forks.as_native() as *mut Box<Fork<CommandContext<()>>>,
            forks_len as usize,
            forks_cap as usize,
        );
        (nodes, executors, tab_completers, forks)
    };
    let game = cx.game_mut();
    let mut dispatcher = game
        .resources
        .get_mut::<CommandDispatcher<CommandCtx>>()
        .unwrap();
    let id = cx.plugin_id();

    dispatcher.add_nodes(nodes);

    for executor in executors.into_iter() {
        dispatcher.add_executor(move |args: &mut Args, mut context: CommandCtx| {
            let plugin_manager = context
                .resources
                .get::<Rc<RefCell<PluginManager>>>()
                .unwrap();
            let rc = plugin_manager.clone();
            drop(plugin_manager);
            let plugin_manager = rc.borrow();
            let plugin = plugin_manager.plugin(id).unwrap();
            plugin.run_command(
                PluginPtrMut::from_native(&executor as *const _ as i64),
                args,
                context,
            )
        });
    }

    let mut fork_now = dispatcher.forks().count();
    for fork in forks {
        let fork = &fork as *const _ as i64;
        fork_now = dispatcher.add_fork(Box::new(
            move |args: &mut Args,
                  context: CommandCtx,
                  mut f: Box<&mut dyn FnMut(&mut Args, CommandCtx) -> CommandOutput>| {
                let plugin_manager = context
                    .resources
                    .get::<Rc<RefCell<PluginManager>>>()
                    .unwrap();
                let rc = plugin_manager.clone();
                drop(plugin_manager);
                let plugin_manager = rc.borrow();
                let plugin = plugin_manager.plugin(id).unwrap();
                plugin.run_command_fork(
                    PluginPtrMut::from_native(fork),
                    args,
                    context,
                    fork_now as u32,
                )
            },
        ) as Box<_>);
    }

    for (key, complete) in tab_completers {
        dispatcher.register_tab_completion(&key, move |text, context| {
            let plugin_manager = context
                .resources
                .get::<Rc<RefCell<PluginManager>>>()
                .unwrap();
            let rc = plugin_manager.clone();
            drop(plugin_manager);
            let plugin_manager = rc.borrow();
            let plugin = plugin_manager.plugin(id).unwrap();
            plugin.run_command_completer(
                PluginPtrMut::from_native(&complete as *const _ as usize as i64),
                text,
                context,
            )
        });
    }
    Ok(())
}
