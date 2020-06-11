//! Loading of recipe files and crafting algorithms ("solving").

use feather_items::Item;

mod model;
mod recipe;
mod solver;

pub use recipe::convert;
pub use solver::{transpose, Solver};

pub const TABLE_WIDTH: usize = 3;
pub const TABLE_SIZE: usize = TABLE_WIDTH * TABLE_WIDTH;

/// A crafting grid. Origin is at the upper left corner.
/// Stored in column-major format. Indexing by [x][y]
/// will yield the item `x` slots from the left
/// and `y` slots from the top.
pub type Grid = [[Option<Item>; TABLE_WIDTH]; TABLE_WIDTH];
