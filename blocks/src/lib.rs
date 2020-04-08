mod prototype;
mod registry;

pub use registry::{BlockRegistry, BlocksStore, BLOCKS};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlockId(u16);

impl BlockId {
    pub fn inner(self) -> u16 {
        self.0
    }
}

impl From<BlockId> for u16 {
    fn from(id: BlockId) -> Self {
        id.inner()
    }
}
