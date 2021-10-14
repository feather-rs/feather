//! Implements the Feather command dispatching framework
//! and vanilla commands not defined by plugins.

use std::ops::{Deref, DerefMut};

use commands::arguments::EntitySelector;
pub use commands::dispatcher::CommandDispatcher;
use commands::dispatcher::CommandOutput;
use log::{debug, info};

use common::Game;
use ecs::Entity;
use libcraft_text::{Text, TextComponentBuilder};
use quill_common::components::{ChatBox, Name};

mod impls;
pub mod utils;

/// Context passed into a command. This value can be used
/// for access to game and entity data, such as components.
pub struct CommandCtx {
    /// The entity which triggered the command.
    ///
    /// _Not necessarily a player_. If the command was executed
    /// from the server console, then this will be the "server entity"
    /// associated with the console. You may check if an entity is a player
    /// by checking if it has the `Player` component. Similarly, you
    /// may check if an entity is the server console through the `Console` component.
    ///
    /// Note that players and the console are not the only possible command senders,
    /// and command implementations should account for this.
    pub sender: Entity,
    /// The game state. We don't want CommandDispatcher to have a lifetime, so
    /// raw pointers are here to elide the lifetime.
    /// Since CommandCtx is not Send, it should be sound
    game: *mut Game,
}

impl Deref for CommandCtx {
    type Target = Game;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.game }
    }
}

impl DerefMut for CommandCtx {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.game }
    }
}

impl CommandCtx {
    pub fn new(game: &mut Game, sender: Entity) -> CommandCtx {
        CommandCtx {
            game: game as *mut Game,
            sender,
        }
    }
    pub fn send_message(&self, message: impl Into<Text>) {
        self.ecs
            .get_mut::<ChatBox>(self.sender)
            .unwrap()
            .send_system(message)
    }
    /// Find entities by selector and report an error if no entities/players were found
    pub fn find_non_empty_entities_by_selector(
        &self,
        selector: &EntitySelector,
        players_only: bool,
    ) -> Option<Vec<Entity>> {
        let entities = self.find_entities_by_selector(selector);
        if entities.is_empty() {
            self.send_message(match (selector, players_only) {
                (EntitySelector::Name(_), true) => Text::translate("argument.player.unknown").red(),
                (_, true) => Text::translate("argument.entity.notfound.player").red(),
                (_, false) => Text::translate("argument.entity.notfound.entity").red(),
            });
            None
        } else {
            Some(entities)
        }
    }
    pub fn find_entities_by_selector(&self, selector: &EntitySelector) -> Vec<Entity> {
        utils::find_entities_by_selector(self.sender, self, selector)
    }
}

pub fn register_vanilla_commands(dispatcher: &mut CommandDispatcher<CommandCtx>) {
    impls::register_all(dispatcher)
}

pub fn dispatch_command(
    dispatcher: &CommandDispatcher<CommandCtx>,
    game: &mut Game,
    sender: Entity,
    command: &str,
    log: bool,
) -> Option<CommandOutput> {
    let ctx = CommandCtx::new(game, sender);

    if dispatcher.find_command(command).is_none() {
        if let Ok(mut chat) = ctx.ecs.get_mut::<ChatBox>(sender) {
            chat.send_system(
                Text::translate("command.unknown.command")
                    .push_extra(
                        Text::Array(vec![
                            Text::of("\n/").gray(),
                            Text::of(command.to_string()).underlined(),
                            Text::translate("command.context.here").italic(),
                        ])
                        .on_click_suggest_command(format!("/{}", command)),
                    )
                    .red(),
            );
        }
        debug!("Unknown command: /{}", command);
        None
    } else {
        if log {
            info!(
                "{} issued server command: /{}",
                ctx.ecs
                    .get::<Name>(sender)
                    .map(|name| (***name).to_string())
                    .unwrap_or_else(|_| "Console".to_owned()),
                command
            );
        }
        let result = dispatcher.execute_command(command, ctx);
        match &result {
            Some(Ok(n)) => debug!("Command result: {:?}", n),
            Some(Err(e)) => debug!("Command error: {}", e),
            None => debug!("Command not found"),
        }
        result
    }
}

pub fn tab_complete(
    dispatcher: &CommandDispatcher<CommandCtx>,
    game: &mut Game,
    sender: Entity,
    prompt: &str,
) -> Vec<(String, Option<String>)> {
    dispatcher
        .tab_complete(prompt, CommandCtx::new(game, sender))
        .unwrap_or_default()
}
