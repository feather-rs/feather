use feather_blocks::BlockId;
use feather_items::Item;

mod mappings;

pub trait ItemToBlock {
    fn to_block(self) -> Option<BlockId>;
}

impl ItemToBlock for Item {
    fn to_block(self) -> Option<BlockId> {
        mappings::item_to_block(self)
    }
}

pub trait BlockToItem {
    fn to_item(self) -> Option<Item>;
}

impl BlockToItem for BlockId {
    fn to_item(self) -> Option<Item> {
        mappings::block_to_item(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_to_block() {
        let items = [
            Item::EnderPearl,
            Item::Stone,
            Item::AcaciaWood,
            Item::Cobblestone,
            Item::Bone,
        ];
        let blocks = [
            None,
            Some(BlockId::stone()),
            Some(BlockId::acacia_wood()),
            Some(BlockId::cobblestone()),
            None,
        ];

        for (item, block) in items.iter().zip(blocks.iter()) {
            assert_eq!(item.to_block(), block.clone());
        }
    }

    #[test]
    fn test_block_to_item() {
        let blocks = [
            BlockId::cobblestone(),
            BlockId::grass_block().with_snowy(true),
            BlockId::grass_block().with_snowy(false),
            BlockId::sandstone(),
        ];

        let items = [
            Item::Cobblestone,
            Item::GrassBlock,
            Item::GrassBlock,
            Item::Sandstone,
        ];

        for (block, item) in blocks.iter().zip(items.iter()) {
            assert_eq!(block.to_item(), Some(*item));
        }
    }
}
