use std::collections::HashMap;

use blocks::BlockKind;

use crate::Game;

#[derive(Default)]
pub struct InteractableRegistry {
    registry: HashMap<BlockKind, usize>,
}

impl InteractableRegistry {
    /// Creates a new, empty [`InteractableRegistry`]
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
        }
    }

    /// Registers that there is a handler that handles interactions
    /// with the [`BlockKind`].
    pub fn register(&mut self, block: BlockKind) {
        let value = self.registry.get(&block).copied();

        match value {
            Some(count) => {
                self.registry.insert(block, count + 1);
            }
            None => {
                self.registry.insert(block, 1);
            }
        }
    }

    /// Deregisters a handler for a block interaction.
    pub fn deregister(&mut self, block: BlockKind) {
        let value = self.registry.get(&block).copied();

        match value {
            Some(count) => {
                if count == 0 {
                    panic!(
                        "Tried to deregister an interaction handler on a block with 0 handlers."
                    );
                } else {
                    self.registry.insert(block, count - 1);
                }
            }
            None => {
                panic!("Tried to deregister an interaction handler on a block with 0 handlers.")
            }
        }
    }

    pub fn is_registered(&self, block: BlockKind) -> bool {
        self.registry.get(&block).is_some()
    }
}

pub fn register(game: &mut Game) {
    game.insert_resource(InteractableRegistry::default());
}
