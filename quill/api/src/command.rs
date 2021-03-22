/*
This file contains defenitions used for quills/feathers command dispatching/calling.
A command is something like "/msg KillerBunny You stole my name!"
*/

use crate::{EntityId, Game};

pub enum Caller {
    Player(EntityId),
    Terminal,
}
/// This is what is passed into the command when its called.
pub struct CommandContext {
    pub game: Game,
    pub caller: Caller,
}
