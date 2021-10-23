//! Implements the Feather command dispatching framework
//! and vanilla commands not defined by plugins.

use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};

use commands::arguments::{BlockPos, EntityAnchor, EntitySelector};
pub use commands::dispatcher::CommandDispatcher;
use commands::dispatcher::{CommandOutput, TabCompletion};
use log::{debug, info};

use common::events::BlockChangeEvent;
use common::Game;
use ecs::Entity;
use feather_base::ValidBlockPosition;
use feather_blocks::BlockId;
use libcraft_core::{BlockPosition, Position, Vec3d};
use libcraft_text::{Text, TextComponentBuilder};
use quill_common::components::{ChatBox, Name, Sneaking};

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
    /// The position at which the command is executed.
    /// This is not necessarily sender's location: it can be modified by /execute at
    pub position: Option<Position>,
    pub anchor: EntityAnchor,
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
    pub fn new(
        game: &mut Game,
        sender: Entity,
        position: Option<Position>,
        anchor: EntityAnchor,
    ) -> CommandCtx {
        CommandCtx {
            game: game as *mut Game,
            sender,
            position,
            anchor,
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
    pub fn block_pos(&self, pos: &BlockPos) -> Result<ValidBlockPosition, BlockPosError> {
        const EYE_LEVEL: f64 = 1.27;
        const EYE_LEVEL_SNEAKING: f64 = 1.62;

        if self.position.is_none()
            && matches!(
                pos,
                BlockPos::Local(..)
                    | BlockPos::Relative { x: (true, _), .. }
                    | BlockPos::Relative { y: (true, _), .. }
                    | BlockPos::Relative { z: (true, _), .. }
            )
        {
            Err(BlockPosError::NotAnEntity)
        } else {
            let pos = match pos {
                BlockPos::Relative { x, y, z } => BlockPosition {
                    x: if x.0 {
                        self.position.unwrap().x.floor() as i32 + x.1
                    } else {
                        x.1
                    },
                    y: if y.0 {
                        self.position.unwrap().y.floor() as i32 + y.1
                    } else {
                        y.1
                    },
                    z: if z.0 {
                        self.position.unwrap().z.floor() as i32 + z.1
                    } else {
                        z.1
                    },
                },
                BlockPos::Local(_, _, _) => {
                    let _origin = match self.anchor {
                        EntityAnchor::Feet => self.position.unwrap(),
                        EntityAnchor::Eyes => {
                            if ***self.ecs.get::<&Sneaking>(self.sender).unwrap() {
                                self.position.unwrap() + Vec3d::new(0.0, EYE_LEVEL_SNEAKING, 0.0)
                            } else {
                                self.position.unwrap() + Vec3d::new(0.0, EYE_LEVEL, 0.0)
                            }
                        }
                    };
                    unimplemented!()
                }
            };

            ValidBlockPosition::try_from(pos).map_err(|_| BlockPosError::InvalidPosition)
        }
    }
    pub fn set_block_at(&mut self, pos: ValidBlockPosition, block: BlockId, destroy: bool) -> bool {
        if !destroy && self.world.block_at(pos) == Some(block) {
            false
        } else {
            if destroy {
                // TODO drop old block, play its destroy sound
            }
            self.world.set_block_at(pos, block);
            self.ecs.insert_event(BlockChangeEvent::single(pos));
            true
        }
    }
}

#[derive(Debug)]
pub enum BlockPosError {
    NotAnEntity,
    InvalidPosition,
}

pub fn register_vanilla_commands(dispatcher: &mut CommandDispatcher<CommandCtx, Text>) {
    impls::register_all(dispatcher)
}

pub fn dispatch_command(
    dispatcher: &mut CommandDispatcher<CommandCtx, Text>,
    game: &mut Game,
    sender: Entity,
    command: &str,
    log: bool,
) -> Option<CommandOutput> {
    let ctx = CommandCtx::new(
        game,
        sender,
        game.ecs.get::<Position>(sender).ok().map(|pos| *pos),
        EntityAnchor::default(),
    );

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
    dispatcher: &CommandDispatcher<CommandCtx, Text>,
    game: &mut Game,
    sender: Entity,
    prompt: &str,
) -> TabCompletion<Text> {
    dispatcher
        .tab_complete(
            prompt,
            CommandCtx::new(
                game,
                sender,
                game.ecs.get::<Position>(sender).ok().map(|pos| *pos),
                EntityAnchor::default(),
            ),
        )
        .unwrap_or_default()
}
