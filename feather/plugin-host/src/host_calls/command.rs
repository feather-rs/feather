use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use commands::dispatcher::{Args, CommandDispatcher, Completer};
use commands::node::CommandNode;
use commands::parser::ArgumentParser;

use feather_commands::CommandCtx;
use feather_plugin_host_macros::host_function;
use quill::command::{Caller, CommandContext};
use quill::Game;
use quill_common::EntityId;

use crate::context::{PluginContext, PluginPtr, PluginPtrMut};
use crate::PluginManager;
use std::collections::HashMap;

#[host_function]
pub fn modify_command_executor(
    cx: &PluginContext,
    nodes: PluginPtrMut<u8>,
    executors: PluginPtrMut<u8>,
    tab_completers: PluginPtrMut<u8>,
) -> anyhow::Result<()> {
    // SAFETY: Plugins should pass valid pointers
    let (nodes, executors, tab_completers) = unsafe {
        let nodes = nodes.as_native() as *mut Vec<Box<CommandNode<Box<dyn ArgumentParser>>>>;
        let executors =
            executors.as_native() as *mut Vec<Box<dyn Fn(Args, CommandContext) -> bool>>;
        let tab_completers = tab_completers.as_native() as *mut HashMap<String, Completer>;
        (
            std::mem::take(&mut *nodes),
            std::mem::take(&mut *executors),
            std::mem::take(&mut *tab_completers),
        )
    };
    let game = cx.game_mut();
    let mut dispatcher = game
        .resources
        .get_mut::<CommandDispatcher<CommandCtx>>()
        .unwrap();

    dispatcher.add_nodes(nodes);
    for executor in executors.into_iter() {
        let id = cx.plugin_id();
        dispatcher.add_executor(Box::new(move |args: Args, mut context: CommandCtx| {
            let plugin_manager = context
                .game
                .resources
                .get::<Rc<RefCell<PluginManager>>>()
                .unwrap();
            let rc = plugin_manager.clone();
            drop(plugin_manager);
            let plugin_manager = rc.borrow();
            let plugin = plugin_manager.plugin(id).unwrap();
            let caller = Some(EntityId(context.sender.to_bits()));
            plugin.enter(&mut *context.game, || {
                executor(
                    args,
                    CommandContext {
                        game: Game::new(),
                        caller: Caller::from(caller),
                    },
                )
            })
        }));
    }
    for (key, value) in tab_completers {
        dispatcher.register_tab_completion(&key, value);
    }
    Ok(())
}
