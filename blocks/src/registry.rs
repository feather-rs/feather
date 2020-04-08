use bit_set::BitSet;
use once_cell::sync::Lazy;
use std::ops::Deref;

macro_rules! block_registry {
    ($($prop:ident: $ty:ident),+) => {
        #[derive(Debug)]
        pub struct BlockRegistry {
            $(
              $prop: attribute_store!($ty),
            )+
            store: BlocksStore,
        }

        impl crate::BlockId {
            $(
                pub fn $prop(self) -> $ty {
                    attribute_access!(self.0 as usize, $prop, $ty)
                }
            )+
        }
    }
}

macro_rules! attribute_store {
    (bool) => {
        BitSet
    };
    ($ty:ident) => {
        Vec<$ty>
    };
}

macro_rules! attribute_access {
    ($index:expr, $prop:ident, bool) => {{
        BLOCKS.$prop.contains($index)
    }};
    ($index:expr, $prop:ident, $ty:ident) => {{
        BLOCKS.$prop[$index]
    }};
}

blocks_store!();
block_registry!(is_passthrough: bool, id_1_13_2: u16);

pub static BLOCKS: Lazy<BlockRegistry> = Lazy::new(|| todo!());

impl Deref for BlockRegistry {
    type Target = BlocksStore;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}
