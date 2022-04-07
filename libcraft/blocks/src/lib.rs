mod block;
pub mod block_data;
pub mod data;
mod registry;
mod simplified_block;
mod utils;

pub use block::BlockKind;
pub use block_data::BlockData;
pub use registry::BlockState;
pub use simplified_block::SimplifiedBlockKind;

pub const HIGHEST_ID: u16 = 20341;
