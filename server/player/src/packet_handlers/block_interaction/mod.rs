mod crafting_table;

pub use crafting_table::CraftingTableInteraction;

pub trait InteractionHandler: Send + Sync {
    fn handle_interaction(&self);
}
