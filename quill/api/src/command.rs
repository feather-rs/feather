/*
This file contains defenitions used for quills/feathers command dispatching/calling.
A command is something like "/msg KillerBunny You stole my name!"
*/

use crate::Entity;
use crate::Game;
use quill_common::EntityId;

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

/// This is what is passed into the command when its called.
pub struct CommandContext {
    pub game: Game,
    pub caller: Caller,
}
