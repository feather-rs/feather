//! Implements the Feather command dispatching framework,
//! based on our `lieutenant` library (a Rust fork
//! of Mojang's [brigadier](https://github.com/Mojang/brigadier).
//!
//! Also implements vanilla commands not defined by plugins.

mod arguments;
mod impls;

use feather_core::text::{Text, TextComponentBuilder};
use feather_server_types::{Game, MessageReceiver};
use fecs::{Entity, World};
use impls::*;
use lieutenant::CommandDispatcher;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

/// Dumb workaround for a certain lifetime issue.
///
/// `CommandCtx` stores references to `Game`, and it
/// is used as the `C` parameter for `CommandDispatcher`,
/// This combination of lifetimes and storage in structs
/// prevents a lifetime-based `CommandCtx` from being stored
/// in `CommandState` without adding a lifetime parameter to `CommandState`.
///
/// Since `CommandCtx` is never actually _stored_ in `CommandState` (it's
/// only passed into a function), we can (hopefully) soundly erase
/// the lifetime parameters. FIXME: if someone has a better solution,
/// a PR is welcome :)
pub struct LifetimelessMut<T>(*mut T);

impl<T> Deref for LifetimelessMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &mut *self.0 }
    }
}

impl<T> DerefMut for LifetimelessMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

unsafe impl<T> Send for LifetimelessMut<T> where T: Send {}
unsafe impl<T> Sync for LifetimelessMut<T> where T: Sync {}

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
    /// The game state.
    pub game: LifetimelessMut<Game>,
    /// The `World`, for access to components.
    pub world: LifetimelessMut<World>,
}

impl lieutenant::Context for CommandCtx {
    type Error = anyhow::Error;
    type Ok = Option<String>;
}

macro_rules! commands {
    ($dispatcher:ident : $($command:expr,)*) => {
        $(
            $dispatcher.register($command).unwrap();
        )*
    }
}

/// State storing all registered commands.
pub struct CommandState {
    dispatcher: Arc<CommandDispatcher<CommandCtx>>,
}

impl Default for CommandState {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandState {
    /// Initializes the command state.
    pub fn new() -> Self {
        let mut dispatcher = CommandDispatcher::<CommandCtx>::new();

        commands! {
            dispatcher:
                tp_1,
                tp_2,
                tp_3,
                tp_4,

                gamemode_1,
                gamemode_2,

                whisper,
                say,
                me,

                kick_1,
                kick_2,

                stop,

                clear_1,
                clear_2,
                clear_3,
                clear_4,

                seed,

                ban_withreason,
                ban_noreason,
                banip_withreason,
                banip_noreason,
                banip_withreason_ip,
                banip_noreason_ip,

                pardon,
                pardonip,

                time_query,
                time_add,
                time_set_0,
                time_set_1,
        }

        Self {
            dispatcher: Arc::new(dispatcher),
        }
    }

    /// Dispatches a command.
    pub fn dispatch(&self, game: &mut Game, world: &mut World, sender: Entity, command: &str) {
        let mut ctx = CommandCtx {
            game: LifetimelessMut(game),
            world: LifetimelessMut(world),
            sender,
        };

        match self.dispatcher.dispatch(&mut ctx, command) {
            Ok(ok) => {
                if let Some(msg) = ok {
                    if let Some(mut receiver) = world.try_get_mut::<MessageReceiver>(sender) {
                        receiver.send(Text::from(msg));
                    }
                }
            }

            Err(errs) => {
                let msg = if let Some(last) = errs.last() {
                    Text::from(last.to_string()).red()
                } else {
                    Text::from("Unknown command.")
                };

                if let Some(mut receiver) = world.try_get_mut::<MessageReceiver>(sender) {
                    receiver.send(msg);
                }
            }
        }
    }
}
