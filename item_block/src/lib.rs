use feather_blocks::Block;
use feather_items::Item;

mod mappings;

pub trait ItemToBlock {
    fn to_block(self) -> Option<Block>;
}

impl ItemToBlock for Item {
    fn to_block(self) -> Option<Block> {
        mappings::item_to_block(self)
    }
}

pub trait BlockToItem {
    fn to_item(self) -> Option<Item>;
}

impl BlockToItem for Block {
    fn to_item(self) -> Option<Item> {
        mappings::block_to_item(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use feather_blocks::{AcaciaWoodAxis, AcaciaWoodData, GrassBlockData};

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
            Some(Block::Stone),
            Some(Block::AcaciaWood(AcaciaWoodData {
                axis: AcaciaWoodAxis::Y,
            })),
            Some(Block::Cobblestone),
            None,
        ];

        for (item, block) in items.iter().zip(blocks.iter()) {
            assert_eq!(item.to_block(), block.clone());
        }
    }

    #[test]
    fn test_block_to_item() {
        let blocks = [
            Block::Cobblestone,
            Block::GrassBlock(GrassBlockData { snowy: true }),
            Block::GrassBlock(GrassBlockData { snowy: false }),
            Block::Sandstone,
        ];

        let items = [
            Item::Cobblestone,
            Item::GrassBlock,
            Item::GrassBlock,
            Item::Sandstone,
        ];

        for (block, item) in blocks.iter().zip(items.iter()) {
            assert_eq!(block.clone().to_item(), Some(*item));
        }
    }
}
