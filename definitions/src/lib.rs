#[macro_use]
extern crate num_derive;

mod generated;

pub use generated::*;

impl Default for BlockKind {
    fn default() -> Self {
        BlockKind::Air
    }
}
