mod block;
pub mod block_data;
pub mod data;
mod registry;
mod utils;
mod simplified_block;

pub use block::BlockKind;
pub use registry::BlockState;
pub use simplified_block::SimplifiedBlockKind;
pub use block_data::BlockData;

pub const HIGHEST_ID: u16 = 20341;