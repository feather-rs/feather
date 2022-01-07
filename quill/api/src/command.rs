/*
This file contains defenitions used for quills/feathers command dispatching/calling.
A command is something like "/msg KillerBunny You stole my name!"
*/

use libcraft_text::Text;
use quill_common::{EntityId, Pointer};

use crate::Entity;
use crate::Game;

#[derive(Debug, Clone)]
#[repr(C)]
pub enum Caller {
    Player(Entity),
    Terminal,
}

impl From<Option<EntityId>> for Caller {
    fn from(it: Option<EntityId>) -> Self {
        match it {
            Some(id) => Caller::Player(Entity::new(crate::EntityId(id))),
            None => Caller::Terminal,
        }
    }
}

impl Caller {
    pub fn send_message(&self, message: impl Into<Text>) {
        unsafe {
            match self {
                Caller::Player(entity) => quill_sys::entity_send_message(
                    entity.id().0,
                    Pointer::new(&message.into() as *const _ as *const _),
                ),
                Caller::Terminal => todo!(),
            }
        }
    }
}

/// This is what is passed into the command when its called.
#[repr(C)]
pub struct CommandContext<'a, Plugin> {
    pub game: Game,
    pub caller: Caller,
    pub plugin: &'a mut Plugin,
}

impl<'a, Plugin> CommandContext<'a, Plugin> {
    pub fn new(game: Game, caller: Caller, plugin: &'a mut Plugin) -> CommandContext<'a, Plugin> {
        CommandContext {
            game,
            caller,
            plugin,
        }
    }
}
