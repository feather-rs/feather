use std::{
    cell::RefCell,
    convert::TryInto,
    fmt::{self, Debug},
    rc::Rc,
};

use crate::{context::PluginPtrMut, Game, PluginId, PluginManager};
use anyhow::bail;
use feather_ecs::{Entity, EntityRef};
use lieutenant::regex::early_termination::CmdPos;
use lieutenant::regex::{dfa::DFA, NFA};
use quill_common::EntityId;

// Constant that should reflect how many commands a default
// server starts with. Vanilla has roughly 60
const NUM_COMMANDS: usize = 20;

type CommandId = u32;

/// A collection of data containing what plugin a command was defined in,
/// the pointer to its Box<Command<...>>, and the regex that determines
/// when the command should be called.(The command does some parsing beyond the regex).
pub struct CommandDescription {
    pub plugin_id: PluginId,
    pub regex: String,
    // Should only come from host_call register_system.
    // it is a pointer to a Box<dyn Command<GameState=(Plugin,CommandContext),z CommandResult=i64>>
    pub callback: PluginPtrMut<u8>,
}

impl CommandDescription {
    // Returns true or false depending on if the command executed or not.
    // If it returns false that means the commands parser did not recognize
    // the commnd.
    /// caller: None => caller was terminal,
    /// caller: Some(x) => player x called command.
    fn call(&self, game: &mut Game, input: &str, caller: Option<EntityId>) -> anyhow::Result<()> {
        let plugin_manager = Rc::clone(&*game.resources.get::<Rc<RefCell<PluginManager>>>()?);
        let plugin_manager = plugin_manager.borrow();
        let plugin = plugin_manager.plugin(self.plugin_id);

        match plugin {
            Some(plugin) => {
                plugin.call_command(game, input, self.callback, caller)?;
                Ok(())
            }
            None => {
                // Plugin has been unloaded.
                bail!("Plugin was not loaded.")
            }
        }
    }
}

pub struct CommandDispatcher {
    dfa: DFA<CmdPos<CommandId>>,
    commands: Vec<CommandDescription>,
}

impl fmt::Debug for CommandDispatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.dfa.fmt(f)
    }
}

impl CommandDispatcher {
    pub fn new() -> Self {
        Self {
            dfa: DFA::new(),
            commands: Vec::with_capacity(NUM_COMMANDS),
        }
    }

    /// Adds a command. Should be followed by update update dfa.
    pub fn add_command(&mut self, cmd: CommandDescription) {
        self.commands.push(cmd);
        // This method should not be called for every command we
        // add. It is somewhat expensive. It is O(n) in the number of
        // commands we have registerd so far. Therefor adding m commands
        // is a O(m^2) operation.
        // Instead we should (option a) try to build only the dfa for the
        // single cmd (to check that its regex does not use any unimplemented features)
        /// by calling  NFA::<CmdPos<CommandId>>::from_command_regex(&cmd.regex,0).is_ok();
        /// or (option b) add a method to lieutenants NFA that checks if the regex
        /// contains any unimplemented features. See lieutenant/regex/regex_to_nfa, and
        /// call that method instead. Anyways for these options we need someway of triggering
        /// dfa updates. Could probably happen inside plugin load function, in main, or a host-call.
        self.update_dfa();
    }

    // Can fail if we run out of u32 ids for the nodes in the nfa graph. This should be unlikely.
    pub fn update_dfa(&mut self) -> anyhow::Result<()> {
        let mut nfa = NFA::<CmdPos<CommandId>>::empty();

        for (id, cmd) in self.commands.iter().enumerate() {
            let cmd_nfa = NFA::<CmdPos<CommandId>>::from_command_regex(
                &cmd.regex,
                id.try_into()
                    .expect("Cant register 2^32 different commands!"),
            )?;
            nfa = nfa.or(cmd_nfa)?;
        }
        // Reduses the number of states/nodes, so it cant fail.
        self.dfa = nfa.into_early_termination_dfa();
        Ok(())
    }

    /// calls the corresponding command (if it exists) and returns the resulting u32
    /// Caller is player calling the command, None means terminal.
    pub fn call(
        &self,
        game: &mut Game,
        input: &str,
        caller: Option<EntityId>,
    ) -> anyhow::Result<i64> {
        match self.dfa.early_termination_find(input) {
            Ok(ids) => {
                for id in ids {
                    let description = &self.commands[id as usize];

                    let plugin_manager =
                        Rc::clone(&*game.resources.get::<Rc<RefCell<PluginManager>>>()?);
                    let plugin_manager = plugin_manager.borrow();
                    let plugin = plugin_manager.plugin(description.plugin_id);

                    match plugin {
                        Some(plugin) => {
                            match plugin.call_command(
                                game,
                                input,
                                description.callback.clone(),
                                caller,
                            ) {
                                Ok(x) => {
                                    // Command sucsessfully ran
                                    return Ok(x);
                                }
                                Err(_) => {
                                    // Was not able to call command, probably due to failed parsing.
                                    // Unless there is some failure case i have not considerd, or someone
                                    // is not using lieutenant, but their own shenanigans.
                                    continue;
                                }
                            }
                        }

                        None => {
                            // Plugin has been unloaded.
                            continue;
                        }
                    }
                }
            }
            Err(_ids) => {
                // Regex did not match, ids show what commands were possible matches unitil parsing failed. Since we are doing
                // early termination this list should contain at least two values.
            }
        }

        bail!("Failed to find a command that matched the input")
    }
}
