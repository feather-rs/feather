pub mod condition;
mod entry;
pub mod function;
pub mod table;

pub use condition::Condition;
pub use entry::Entry;
pub use function::Function;
pub use table::LootTable;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExactOrRandom<T> {
    Exact(T),
    Random { min: T, max: T },
}
